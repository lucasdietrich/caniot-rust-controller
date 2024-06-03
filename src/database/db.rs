use log::info;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{ConnectOptions, PgPool};

use super::SettingsHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            connection_string: "postgres://caniot:caniot@localhost/caniot".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Database, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;

        Ok(Database { pool })
    }

    pub async fn initialize_tables(&self) -> Result<(), sqlx::Error> {
        info!("Initializing database tables");

        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }

    pub fn get_settings_handle(&self) -> SettingsHandle {
        SettingsHandle::new(&self.pool)
    }
}
