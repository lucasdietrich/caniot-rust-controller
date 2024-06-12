use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::settings::SettingTrait;
use thiserror::Error;

use super::Database;

pub struct SettingsStore<'a>(&'a PgPool);

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Setting not found")]
    NotFound,
    #[error("Setting type mismatch")]
    TypeMismatch,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> SettingsStore<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }

    pub async fn read<T: SettingTrait>(&self, key: &str) -> Result<T, SettingsError> {
        let type_name = T::type_name();
        let row = sqlx::query("SELECT val FROM settings WHERE key = $1 AND type = $2 LIMIT 1")
            .bind(key)
            .bind(type_name)
            .fetch_one(self.0)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => SettingsError::NotFound,
                _ => SettingsError::DatabaseError(e),
            })?;

        let val: String = row.try_get("val")?;
        let value = T::try_from_str(&val).ok_or(SettingsError::TypeMismatch)?;

        Ok(value)
    }

    pub async fn set<T: SettingTrait>(&self, key: &str, value: &T) -> Result<(), SettingsError> {
        sqlx::query(
            "INSERT INTO settings (key, val, type) VALUES ($1, $2, $3) ON CONFLICT (key) DO UPDATE SET val = $2, type = $3"
        )
        .bind(key)
        .bind(value.as_string())
        .bind(T::type_name())
        .execute(self.0)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), SettingsError> {
        sqlx::query("DELETE FROM settings WHERE key = $1")
            .bind(key)
            .execute(self.0)
            .await?;

        Ok(())
    }

    pub async fn delete_all(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM settings").execute(self.0).await?;

        Ok(())
    }
}
