pub mod db;
pub mod settings_store;

pub use db::*;
pub use settings_store::*;

pub mod settings_types;

#[cfg(test)]
mod settings_store_test;

// Database backend fixed to SQLite
pub type DatabaseType = sqlx::Sqlite;
