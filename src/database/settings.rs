use sqlx::PgPool;

use super::Database;

pub struct Settings<'a>(&'a PgPool);

impl<'a> Settings<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let row = sqlx::query!("SELECT val FROM settings WHERE key = $1", key)
            .fetch_one(self.0)
            .await
            .ok()?;

        row.val
    }

    pub async fn set(&self, key: &str, val: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO settings (key, val) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET val = $2",
            key,
            val
        )
        .execute(self.0)
        .await?;

        Ok(())
    }
}
