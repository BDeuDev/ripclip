use std::sync::{Arc, Mutex};

use crate::db::models::Clip;
use crate::db::queries::{CREATE_TABLE_CLIPS, INSERT_CONTENT_CLIP, SELECT_RECENT_CLIPS};

use sqlx::{Result, SqlitePool};

pub struct ClipRepository {
    pool: SqlitePool,
}

impl ClipRepository {
    pub async fn new(path: &str) -> Result<Self> {
        let pool = SqlitePool::connect(path).await?;
        Ok(Self { pool })
    }

    pub async fn init_table(&self) -> Result<()> {
        sqlx::query(CREATE_TABLE_CLIPS).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn save(&self, content: &str) -> Result<()> {
        sqlx::query(INSERT_CONTENT_CLIP).bind(content).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn recent(&self, limit: i64) -> Result<Vec<Clip>> {
        let clips = sqlx::query_as::<_, Clip>(SELECT_RECENT_CLIPS)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        Ok(clips)
    }
}
