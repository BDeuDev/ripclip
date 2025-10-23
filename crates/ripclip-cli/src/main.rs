use ripclip_core::ipc::IpcStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut stream = IpcStream::connect("/tmp/ripclip.sock")?;
    stream.write_all(b"GET_COUNT")?;
    let mut buf = [0; 4096];
    let len = stream.read(&mut buf).unwrap();
    println!("Respuesta: {}", String::from_utf8_lossy(&buf[..len]));
    Ok(())
}
