use anyhow::Ok;
use ripclip_core::db::repositories::ClipRepository;
use std::{
    fs::create_dir_all,
    sync::{Arc, Mutex},
};
use tokio::{join, task};

use crate::workers::{ClipboardWorker, IPCWorker};

mod workers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    create_dir_all("data")?;

    let repo = Arc::new(ClipRepository::new("data/clipboard.db").await?);

    repo.init_table().await?;

    println!("Daemon iniciado. Escuchando cambios...");

    let repo_clip = Arc::clone(&repo);
    let clipboard_task = task::spawn(async move {
        ClipboardWorker::new(Arc::clone(&repo_clip), 100).run().await;
    });

    let repo_ipc = Arc::clone(&repo);
    let ipc_task = task::spawn(async move {
        IPCWorker::new(repo_ipc).run().await;
    });

    let (res1, res2) = join!(clipboard_task, ipc_task);

    res1?;
    res2?;
    Ok(())
}
