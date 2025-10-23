use std::sync::{Arc, Mutex};

use crate::db::models::Clip;
use crate::db::queries::{CREATE_TABLE_CLIPS, INSERT_CONTENT_CLIP, SELECT_RECENT_CLIPS};
use rusqlite::{Connection, Result, params};

pub struct ClipRepository {
    conn: Arc<Mutex<Connection>>,
}

impl<'a> ClipRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub fn init_table(&self) -> Result<()> {
        let conn = self.conn.lock().expect("No se pudo obtener la conexión a la base de datos");
        conn.execute(CREATE_TABLE_CLIPS, [])?;
        Ok(())
    }

    pub fn save(&self, content: &str) -> Result<()> {
        let conn = self.conn.lock().expect("No se pudo obtener la conexión a la base de datos");
        conn.execute(INSERT_CONTENT_CLIP, params![content])?;
        Ok(())
    }

    pub fn recent(&self, limit: usize) -> Result<Vec<Clip>> {
        let conn = self.conn.lock().expect("No se pudo obtener la conexión a la base de datos");
        let mut stmt = conn.prepare(SELECT_RECENT_CLIPS)?;
        let clips = stmt
            .query_map(params![limit], |row| {
                Ok(Clip { id: row.get(0)?, content: row.get(1)?, copied_at: row.get(2)? })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(clips)
    }
}
