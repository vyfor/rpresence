#![feature(pattern)]

mod ipc;
mod json;
pub mod rpc;

use std::{
    io::{self},
    sync::{Arc, Condvar, Mutex, RwLock},
    thread::{self, JoinHandle},
};

pub use ipc::client::Connection;
use rpc::packet::{Activity, Packet};

#[cfg(target_os = "windows")]
pub struct RichClient<'a> {
    pub client_id: u64,
    pub pid: u32,
    connection_state: Arc<RwLock<ConnectionState>>,
    on_ready: Arc<Option<Box<dyn Fn() + Send + Sync>>>,
    on_disconnect: Arc<Option<Box<dyn Fn() + Send + Sync>>>,
    on_update: Arc<Option<Box<dyn Fn() + Send + Sync>>>,
    last_activity: Option<Activity<'a>>,
    signal: Arc<(Mutex<bool>, Condvar)>,
    handle: Option<JoinHandle<Option<String>>>,
    pipe: Arc<Option<std::fs::File>>,
}

#[cfg(not(target_os = "windows"))]
pub struct RichClient<'a> {
    pub client_id: u64,
    pub pid: u32,
    connection_state: Arc<RwLock<ConnectionState>>,
    on_ready: Arc<Option<Box<dyn Fn() + Send + Sync>>>,
    on_disconnect: Arc<Option<Box<dyn Fn() + Send + Sync>>>,
    on_update: Arc<Option<Box<dyn Fn() + Send + Sync>>>,
    last_activity: Option<Activity<'a>>,
    signal: Arc<(Mutex<bool>, Condvar)>,
    pipe: Arc<RwLock<Option<std::os::unix::net::UnixStream>>>,
}

impl<'a> RichClient<'a> {
    pub fn new(client_id: u64) -> Self {
        Self {
            client_id,
            connection_state: Arc::default(),
            on_ready: Arc::default(),
            on_disconnect: Arc::default(),
            on_update: Arc::default(),
            pipe: Arc::default(),
            last_activity: None,
            pid: std::process::id(),
            handle: None,
            signal: Arc::default(),
        }
    }

    pub fn connect(&mut self, should_block: bool) -> io::Result<()> {
        if *self.connection_state.read().unwrap()
            != ConnectionState::Disconnected
        {
            return Ok(());
        }

        self.open()?;
        *self.connection_state.write().unwrap() = ConnectionState::Connected;
        self.handshake()?;
        self.listen();

        if should_block {
            let (lock, cvar) = &*self.signal;
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
            *started = false;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.perform_check()?;

        self.write(
            1,
            Some(Packet::new(self.pid, None).to_json().unwrap().as_bytes()),
        )
    }

    pub fn update(&mut self, activity: Activity<'a>) -> io::Result<()> {
        println!("update");
        self.perform_check()?;
        println!("perform_check");
        if self.last_activity.as_ref() != Some(&activity) {
            self.write(
                1,
                Some(
                    Packet::new(self.pid, Some(&activity))
                        .to_json()
                        .unwrap()
                        .as_bytes(),
                ),
            )?;
            self.last_activity = Some(activity);
        }

        Ok(())
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        if *self.connection_state.read().unwrap()
            == ConnectionState::Disconnected
        {
            return Ok(());
        }

        self.close()?;
        let mut state = self.connection_state.write().unwrap();
        *state = ConnectionState::Disconnected;
        self.last_activity = None;

        if let Some(on_disconnect) = self.on_disconnect.as_ref() {
            on_disconnect();
        }

        Ok(())
    }

    fn handshake(&mut self) -> io::Result<()> {
        self.write(
            0,
            Some(
                format!("{{\"v\": 1,\"client_id\":\"{}\"}}", self.client_id)
                    .as_bytes(),
            ),
        )
    }

    fn listen(&mut self) {
        let client_id = self.client_id;
        let signal = Arc::clone(&self.signal);
        let connection_state = Arc::clone(&self.connection_state);
        let pipe = Arc::clone(&self.pipe);
        let on_ready = Arc::clone(&self.on_ready);
        let on_disconnect = Arc::clone(&self.on_disconnect);
        let on_update = Arc::clone(&self.on_update);
        self.handle = Some(thread::spawn(move || {
            while *connection_state.read().unwrap()
                != ConnectionState::Disconnected
            {
                let (op, data) = match RichClient::read(&pipe) {
                    Ok(data) => data,
                    Err(_) => return None,
                };
                println!("op: {}", op);

                if *connection_state.read().unwrap()
                    == ConnectionState::Disconnected
                {
                    break;
                }

                println!(
                    "message: {}",
                    String::from_utf8(data.clone()).unwrap()
                );

                match op {
                    1 => {
                        if *connection_state.read().unwrap()
                            == ConnectionState::Connected
                        {
                            *connection_state.write().unwrap() =
                                ConnectionState::SentHandshake;
                            *signal.0.lock().unwrap() = true;
                            signal.1.notify_one();

                            if let Some(on_ready) = on_ready.as_ref() {
                                on_ready();
                            }

                            continue;
                        }

                        if let Some(on_update) = on_update.as_ref() {
                            on_update();
                        }
                    }
                    2 => {
                        if String::from_utf8(data)
                            .unwrap()
                            .contains("Invalid Client ID")
                        {
                            return Some("Invalid Client ID".to_string());
                        }
                        if *connection_state.read().unwrap()
                            != ConnectionState::Disconnected
                        {
                            *connection_state.write().unwrap() =
                                ConnectionState::Disconnected;
                            let _ = RichClient::_close(&pipe, client_id);
                            signal.1.notify_one();
                            if let Some(on_disconnect) = on_disconnect.as_ref()
                            {
                                on_disconnect();
                            }
                        }
                    }
                    _ => {}
                }
            }

            None
        }));
    }

    fn perform_check(&mut self) -> io::Result<()> {
        if let Some(handle) = self.handle.take() {
            if handle.is_finished() {
                if let Ok(Some(err)) = handle.join() {
                    return Err(io::Error::new(io::ErrorKind::Other, err));
                }
            }
        };

        Ok(())
    }
}

#[derive(PartialEq, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connected,
    SentHandshake,
}
