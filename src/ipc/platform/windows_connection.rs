use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io;
use std::os::windows::io::AsRawHandle;

use crate::ipc::client::Connection;
use crate::RichClient;

extern "system" {
    fn CloseHandle(hObject: *mut c_void) -> i32;
}

impl Connection for RichClient<'_> {
    fn open(&mut self) -> io::Result<()> {
        for i in 0..10 {
            match OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("\\\\.\\pipe\\discord-ipc-{i}"))
            {
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
            unsafe {
                CloseHandle(pipe.as_raw_handle());
            }
        }

        Ok(())
    }

    fn _close(pipe: &Option<std::fs::File>, client_id: u64) -> io::Result<()> {
        RichClient::_write(
            pipe,
            2,
            Some(format!("{{'v': 1, 'client_id': {}}}", client_id).as_bytes()),
        )?;
        if let Some(pipe) = pipe.as_ref().as_mut().take() {
            unsafe {
                CloseHandle(pipe.as_raw_handle());
            }
        }

        Ok(())
    }
}
