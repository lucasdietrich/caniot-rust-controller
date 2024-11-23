pub mod db;
pub mod setting;
pub mod settings_store;

pub use db::*;
pub use settings_store::*;
use sqlx::Pool;

pub mod settings_types;

#[cfg(test)]
mod settings_store_test;

#[cfg(feature = "db-postgres")]
pub type DatabaseType = sqlx::Postgres;
#[cfg(feature = "db-sqlite")]
pub type DatabaseType = sqlx::Sqlite;
