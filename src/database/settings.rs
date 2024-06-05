use sqlx::{PgPool, Row};

use super::Database;

pub struct SettingsHandle<'a>(&'a PgPool);

impl<'a> SettingsHandle<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let row = sqlx::query("SELECT val FROM settings WHERE key = $1")
            .bind(key)
            .fetch_one(self.0)
            .await
            .ok()?;

        row.try_get("val").ok()
    }

    pub async fn set(&self, key: &str, val: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO settings (key, val) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET val = $2"
        )
        .bind(key)
        .bind(val)
        .execute(self.0)
        .await?;

        Ok(())
    }
}
