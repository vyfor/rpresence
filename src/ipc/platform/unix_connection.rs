use std::env::var;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

use crate::ipc::client::Connection;
use crate::ipc::utils;
use crate::rpc::packet::Packet;
use crate::RichClient;

impl Connection for RichClient<'_> {
    fn open(&mut self) -> io::Result<()> {
        let dirs = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"]
            .iter()
            .filter_map(|&dir| var(dir).ok())
            .chain(["/tmp".to_string()])
            .flat_map(|base| {
                [
                    base.to_string(),
                    format!("{}/app/com.discordapp.Discord", base),
                    format!("{}/snap.discord", base),
                ]
            });

        for dir in dirs {
            for i in 0..10 {
                match UnixStream::connect(format!("{dir}/discord-ipc-{i}")) {
                    Ok(pipe) => {
                        self.pipe = Some(pipe).into();
                        return Ok(());
                    }
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => continue,
                        _ => return Err(e),
                    },
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found"))
    }

    fn close(&mut self) -> io::Result<()> {
        self.write(
            2,
            Some(
                format!("{{'v': 1, 'client_id': {}}}", self.client_id)
                    .as_bytes(),
            ),
        )?;
        if let Some(pipe) = self.pipe.as_ref().as_ref().as_mut().take() {
            let _ = pipe.shutdown(std::net::Shutdown::Both);
        }

        Ok(())
    }

    fn _close(
        pipe: &Option<std::os::unix::net::UnixStream>,
        client_id: u64,
    ) -> io::Result<()> {
        RichClient::_write(
            pipe,
            2,
            Some(format!("{{'v': 1, 'client_id': {}}}", client_id).as_bytes()),
        )?;
        if let Some(pipe) = pipe.as_ref().as_ref().as_mut().take() {
            let _ = pipe.shutdown(std::net::Shutdown::Both);
        }

        Ok(())
    }
}
