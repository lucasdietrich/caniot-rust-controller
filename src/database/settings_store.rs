use sqlx::{Pool, Row};

use crate::database::settings_types::SettingTrait;
use thiserror::Error;

use super::DatabaseType;

pub struct SettingsStore<'a>(&'a Pool<DatabaseType>);

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
    pub fn new(pool: &'a Pool<DatabaseType>) -> Self {
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

    pub async fn read_or_setting_default<T: SettingTrait>(
        &self,
        key: &str,
    ) -> Result<T, SettingsError> {
        match self.read(key).await {
            Ok(value) => Ok(value),
            Err(SettingsError::NotFound) => Ok(T::default()),
            Err(e) => Err(e),
        }
    }

    pub async fn read_or<T: SettingTrait>(
        &self,
        key: &str,
        default: T,
    ) -> Result<T, SettingsError> {
        match self.read(key).await {
            Ok(value) => Ok(value),
            Err(SettingsError::NotFound) => Ok(default),
            Err(e) => Err(e),
        }
    }

    pub async fn write<T: SettingTrait>(&self, key: &str, value: &T) -> Result<(), SettingsError> {
        sqlx::query(
            "INSERT INTO settings (key, val, type, last_updated) VALUES ($1, $2, $3, datetime('now')) ON CONFLICT (key) DO UPDATE SET val = $2, type = $3, last_updated = datetime('now')",
        )
        .bind(key)
        .bind(value.as_string())
        .bind(T::type_name())
        .execute(self.0)
        .await?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn set_default<T: SettingTrait>(&self, key: &str) -> Result<(), SettingsError> {
        self.write::<T>(key, &T::default()).await
    }

    #[allow(unused)]
    pub async fn delete(&self, key: &str) -> Result<(), SettingsError> {
        sqlx::query("DELETE FROM settings WHERE key = $1")
            .bind(key)
            .execute(self.0)
            .await?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn delete_all(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM settings").execute(self.0).await?;

        Ok(())
    }
}
