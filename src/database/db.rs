use std::time::Duration;

use log::{info, warn};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::time::sleep;

use super::SettingsStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
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
            connection_string: "postgres://caniot:caniot@localhost/caniot".to_string(),
            retries: None,
            retry_interval: 5,
        }
    }
}

#[derive(Debug)]
pub struct Database {
    pub pool: PgPool,
}

const PG_MAX_CONNECTIONS: u32 = 5;

impl Database {
    pub async fn try_connect(config: &DatabaseConfig) -> Result<Database, sqlx::Error> {
        let mut tries = 0;
        loop {
            match PgPoolOptions::new()
                .max_connections(PG_MAX_CONNECTIONS)
                .connect(&config.connection_string)
                .await
            {
                Ok(pool) => {
                    info!("Connected to database");
                    return Ok(Database { pool });
                }
                Err(e) => {
                    if let Some(retries) = config.retries {
                        if tries >= retries {
                            return Err(e);
                        }
                    }
                    tries += 1;
                    warn!(
                        "Failed to connect to database, retrying in {} seconds (retry {}/{})",
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
