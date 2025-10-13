use rusqlite::{Connection, params, Result};
use crate::db::queries::{CREATE_TABLE_CLIPS, INSERT_CONTENT_CLIP, SELECT_RECENT_CLIPS};
use crate::db::models::Clip;

pub struct ClipRepository<'a> {
    conn: &'a Connection,
}

impl<'a> ClipRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn init_table(&self) -> Result<()> {
        self.conn.execute(CREATE_TABLE_CLIPS, [])?;
        Ok(())
    }

    pub fn save(&self, content: &str) -> Result<()> {
        self.conn.execute(INSERT_CONTENT_CLIP, params![content])?;
        Ok(())
    }

    pub fn recent(&self, limit: usize) -> Result<Vec<Clip>> {
        let mut stmt = self.conn.prepare(SELECT_RECENT_CLIPS)?;
        let clips = stmt.query_map(params![limit], |row| {
            Ok(Clip {
                id: row.get(0)?,
                content: row.get(1)?,
                copied_at: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        Ok(clips)
    }
}
