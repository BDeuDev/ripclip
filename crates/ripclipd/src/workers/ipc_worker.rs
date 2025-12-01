use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    sync::Arc,
    thread,
};

use ripclip_core::{db::repositories::ClipRepository, ipc::IpcListener};

pub struct IPCWorker {
    repo: Arc<ClipRepository>,
}

impl IPCWorker {
    pub fn new(repo: Arc<ClipRepository>) -> Self {
        Self { repo }
    }

    pub async fn run(&self) {
        let path = "/tmp/ripclip.sock";

        // Intentamos crear el listener (limpiando el socket anterior si existe)
        let listener = match IpcListener::bind(path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error al crear IPC listener: {e}");
                return;
            }
        };

        println!("IPC Worker escuchando en {path}");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let repo = self.repo.clone();
                    thread::spawn(async move || {
                        if let Err(e) = Self::handle_client(repo, stream).await {
                            eprintln!("Error manejando cliente IPC: {e}");
                        }
                    });
                }
                Err(e) => eprintln!("Error aceptando conexión IPC: {e}"),
            }
        }
    }

    async fn handle_client(repo: Arc<ClipRepository>, mut stream: UnixStream) -> std::io::Result<()> {
        let mut buf = [0u8; 1024];
        let len = stream.read(&mut buf)?;
        if len == 0 {
            return Ok(());
        }

        let input = String::from_utf8_lossy(&buf[..len]).trim().to_string();
        println!("Comando recibido: {input}");

        let response = match input.as_str() {
            "PING" => "PONG".to_string(),
            //change this
            "GET_COUNT" => {
                let count = repo.recent(10).await;
                format!("Cantidad de clips: {:?}", count)
            }
            _ => "Comando no reconocido".to_string(),
        };

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
