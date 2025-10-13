use rusqlite::Connection;
use arboard::Clipboard;
use ripclip_core::db::repositories::ClipRepository;
use std::{fs::create_dir_all, thread, time::Duration};

fn main() -> anyhow::Result<()> {
    create_dir_all("data")?;

    let conn = Connection::open("data/clipboard.db")?;
    let repo = ClipRepository::new(&conn);
    repo.init_table()?;

    let mut clipboard = Clipboard::new()?;

    println!("Daemon iniciado. Escuchando cambios...");
    
    let mut last = clipboard.get_text()?;
    loop {
        if let Ok(current) = clipboard.get_text() {
            if current != last {
                println!("📋 Nuevo texto copiado: {}", current);
                repo.save(&current)?;
                last = current;

                let recents = repo.recent(10)?;
                for r in recents {
                    println!("textos: {:?}", r.content )
                }
                
            }
        }

        println!("Clipboard text was: {}", clipboard.get_text().unwrap());
        thread::sleep(Duration::from_millis(500));
    }
}
