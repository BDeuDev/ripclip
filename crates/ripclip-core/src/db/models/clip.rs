use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Clip {
    pub id: i64,
    pub content: String,
    pub copied_at: String,
}
