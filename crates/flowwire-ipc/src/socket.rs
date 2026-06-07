use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use anyhow::Result;

/// Simple Unix socket wrapper
pub struct IpcServer {
    listener: UnixListener,
}

impl IpcServer {
    pub fn new(path: &str) -> Result<Self> {
        let listener = UnixListener::bind(path)?;
        Ok(Self { listener })
    }

    pub fn accept(&self) -> Result<UnixStream> {
        let (stream, _) = self.listener.accept()?;
        Ok(stream)
    }
}

pub fn send(stream: &mut UnixStream, data: &[u8]) -> Result<()> {
    stream.write_all(data)?;
    Ok(())
}

pub fn receive(stream: &mut UnixStream) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf)?;
    buf.truncate(n);
    Ok(buf)
}
