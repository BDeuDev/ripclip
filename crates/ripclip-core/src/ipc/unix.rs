use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

pub struct IpcListener {
    inner: UnixListener,
}

pub struct IpcStream {
    stream: UnixStream,
}

impl IpcListener {
    pub fn bind(path: &str) -> std::io::Result<Self> {
        let _ = std::fs::remove_file(path);
        Ok(Self { inner: UnixListener::bind(path)? })
    }

    pub fn accept(&self) -> std::io::Result<IpcStream> {
        let (stream, _) = self.inner.accept()?;
        Ok(IpcStream { stream })
    }
    pub fn incoming(&self) -> std::os::unix::net::Incoming<'_> {
        self.inner.incoming()
    }
}

impl IpcStream {
    pub fn connect(path: &str) -> std::io::Result<Self> {
        Ok(Self { stream: UnixStream::connect(path)? })
    }
}

impl Read for IpcStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for IpcStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
