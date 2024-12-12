use std::time::Duration;

use log::{info, warn};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, pool::PoolOptions, Pool, Sqlite};
use tokio::time::sleep;

use super::{DatabaseType, SettingsStore};

#[cfg(feature = "db-postgres")]
pub const DEFAULT_CONNECTION_STRING: &str = "postgres://caniot:caniot@localhost/caniot";
#[cfg(feature = "db-sqlite")]
pub const DEFAULT_CONNECTION_STRING: &str = "sqlite:./caniot.db";
// pub const DEFAULT_CONNECTION_STRING: &str = "sqlite::memory:";

pub fn get_default_connection_string() -> String {
    DEFAULT_CONNECTION_STRING.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "get_default_connection_string")]
    pub connection_string: String,

    // Number of times to retry a failed connection
    // - None: retry indefinitely
    // - Some(n): retry n times
    pub retries: Option<u32>,

    // Retry interval in seconds
    pub retry_interval: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            connection_string: get_default_connection_string(),
            retries: None,
            retry_interval: 5,
        }
    }
}

#[derive(Debug)]
pub struct Storage {
    pub pool: Pool<DatabaseType>,
}

const PG_MAX_CONNECTIONS: u32 = 5;

impl Storage {
    pub async fn try_connect(config: &DatabaseConfig) -> Result<Storage, sqlx::Error> {
        // Create sqlite database if it does not exist
        #[cfg(feature = "db-sqlite")]
        if !Sqlite::database_exists(&config.connection_string).await? {
            warn!("Database does not exist, creating it");
            Sqlite::create_database(&config.connection_string).await?;
            info!("Database created");
        }

        let mut tries = 0;
        loop {
            match PoolOptions::new()
                .max_connections(PG_MAX_CONNECTIONS)
                .connect(&config.connection_string)
                .await
            {
                Ok(pool) => {
                    info!("Connected to database");
                    return Ok(Storage { pool });
                }
                Err(e) => {
                    if let Some(retries) = config.retries {
                        if tries >= retries {
                            return Err(e);
                        }
                    }
                    tries += 1;
                    warn!(
                        "Failed to connect to database: {}, retrying in {} seconds (retry {}/{})",
                        e,
                        config.retry_interval,
                        tries,
                        match config.retries {
                            Some(retries) => retries.to_string(),
                            None => "oo".to_string(),
                        }
                    );
                    sleep(Duration::from_secs(config.retry_interval as u64)).await;
                }
            }
        }
    }

    pub async fn initialize_tables(&self) -> Result<(), sqlx::Error> {
        info!("Initializing database tables");

        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }

    pub fn get_settings_store(&self) -> SettingsStore {
        SettingsStore::new(&self.pool)
    }
}
