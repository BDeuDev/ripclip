use ripclip_core::ipc::{IpcStream};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut stream = IpcStream::connect("/tmp/ripclip.sock")?;
    stream.write_all(b"Mensaje desde CLI")?;
    Ok(())
}