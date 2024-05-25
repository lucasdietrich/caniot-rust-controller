use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};

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

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Database, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;

        Ok(Database { pool })
    }
}
