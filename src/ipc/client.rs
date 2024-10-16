use std::io::{self, Read, Write};

use crate::RichClient;

use super::utils;

impl<'a> RichClient<'a> {
    pub(crate) fn write(
        &mut self,
        opcode: u32,
        data: Option<&[u8]>,
    ) -> io::Result<()> {
        self.pipe.as_ref().as_ref().map_or(
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found")),
            |mut pipe| {
                println!(
                    "Sending packet: op={:?}; data={:?}",
                    opcode,
                    match data {
                        Some(data) => String::from_utf8(data.to_vec()).unwrap(),
                        None => "None".to_string(),
                    }
                );
                let payload = match data {
                    Some(packet) => {
                        let mut payload =
                            utils::encode(opcode, packet.len() as u32);
                        payload.extend_from_slice(packet);
                        payload
                    }
                    None => utils::encode(opcode, 0),
                };
                pipe.write_all(&payload)
            },
        )
    }

    pub(crate) fn _write(
        #[cfg(target_os = "windows")] pipe: &Option<std::fs::File>,
        #[cfg(not(target_os = "windows"))] pipe: &mut Option<
            std::os::unix::net::UnixStream,
        >,
        opcode: u32,
        data: Option<&[u8]>,
    ) -> io::Result<()> {
        pipe.as_ref().map_or(
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found")),
            |mut pipe| {
                let payload = match data {
                    Some(packet) => {
                        let mut payload =
                            utils::encode(opcode, packet.len() as u32);
                        payload.extend_from_slice(packet);
                        payload
                    }
                    None => utils::encode(opcode, 0),
                };
                pipe.write_all(&payload)?;
                Ok(())
            },
        )
    }

    pub(crate) fn read(
        #[cfg(target_os = "windows")] pipe: &Option<std::fs::File>,
        #[cfg(not(target_os = "windows"))] pipe: &Option<
            std::os::unix::net::UnixStream,
        >,
    ) -> io::Result<(u32, Vec<u8>)> {
        pipe.as_ref().map_or(
            Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found")),
            |mut pipe| {
                let mut header = [0; 8];
                pipe.read_exact(&mut header)?;
                let (op, len) = utils::decode(&header);
                let mut buffer = vec![0u8; len as usize];
                pipe.read_exact(&mut buffer)?;
                Ok((op, buffer))
            },
        )
    }
}

pub trait Connection {
    fn open(&mut self) -> io::Result<()>;
    fn close(&mut self) -> io::Result<()>;
    fn _close(
        #[cfg(target_os = "windows")] pipe: &Option<std::fs::File>,
        #[cfg(not(target_os = "windows"))] pipe: &Option<
            std::os::unix::net::UnixStream,
        >,
        client_id: u64,
    ) -> io::Result<()>;
}
