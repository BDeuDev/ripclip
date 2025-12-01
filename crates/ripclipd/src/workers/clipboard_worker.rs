use std::{sync::Arc, time::Duration};

use arboard::Clipboard;
use ripclip_core::db::repositories::ClipRepository;
use tokio::time::sleep;

pub struct ClipboardWorker {
    sleep_time: u64,
    repo: Arc<ClipRepository>,
}

impl ClipboardWorker {
    pub fn new(repo: Arc<ClipRepository>, sleep_time: u64) -> Self {
        Self { repo, sleep_time }
    }

    pub async fn run(&self) {
        let mut clipboard = Clipboard::new().expect("Unexpected Error initializing the Clipboard");

        let mut last = String::new();

        match clipboard.get_text() {
            Ok(v) => last = v,
            Err(e) => {
                println!("Error getting value from clipboard: {}", e);
            }
        }

        loop {

            if let Ok(current) = clipboard.get_text() {
                if !current.is_empty() && current != last {
                    println!("📋 Nuevo texto copiado: {}", current);
                    let _ = self.repo.save(&current).await;
                    last = current;

                    let recents = self.repo.recent(10).await;

                    match recents {
                        Ok(recents) => {
                            for r in recents {
                        println!("textos: {:?}", r.content)
                    }
                        },
            Err(e) => {
                println!("Error getting recents Clips: {}", e);
            }
                    }
                

                }
            }

            sleep(Duration::from_millis(self.sleep_time)).await;
        }
    }
}
