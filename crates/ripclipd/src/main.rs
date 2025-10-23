use anyhow::Ok;
use ripclip_core::db::repositories::ClipRepository;
use rusqlite::Connection;
use std::{
    fs::create_dir_all,
    sync::{Arc, Mutex}
};
use tokio::{join, task};

use crate::workers::{ClipboardWorker, IPCWorker};

mod workers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    create_dir_all("data")?;

    let conn = Arc::new(Mutex::new(Connection::open("data/clipboard.db")?));
    let repo = Arc::new(ClipRepository::new(conn.clone()));

    repo.init_table()?;

    println!("Daemon iniciado. Escuchando cambios...");

    let repo_clip = repo.clone();
    let clipboard_task = task::spawn(async move {
        ClipboardWorker::new(repo_clip, 5000).run().await;
    });

    let repo_ipc = repo.clone();
    let ipc_task = task::spawn(async move {
        IPCWorker::new(repo_ipc).run();
    });

    let (res1, res2) = join!(clipboard_task, ipc_task);

    res1?;
    res2?;
    Ok(())
}
