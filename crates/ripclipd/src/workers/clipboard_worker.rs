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

    pub async fn run(&self){
        let mut clipboard = Clipboard::new().unwrap();

        let mut last = clipboard.get_text().unwrap();
        loop {
            if let Ok(current) = clipboard.get_text() {
                if current != last {
                    println!("📋 Nuevo texto copiado: {}", current);
                    self.repo.save(&current).unwrap();
                    last = current;

                    let recents = self.repo.recent(10).unwrap();
                    for r in recents {
                        println!("textos: {:?}", r.content)
                    }
                }
            }

            println!("Clipboard text was: {}", clipboard.get_text().unwrap());
            sleep(Duration::from_millis(self.sleep_time)).await;
        }
    }
}
