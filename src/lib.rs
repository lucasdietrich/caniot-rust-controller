// library to be used by example or external code (e.g. examples)
#[macro_use]
extern crate rocket;

pub mod bus;
pub mod caniot;
pub mod config;
pub mod controller;
#[cfg(feature = "ble-copro")]
pub mod copro;
pub mod database;
pub mod grpcserver;
pub mod init;
pub mod internal;
pub mod logger;
pub mod shared;
pub mod shutdown;
pub mod utils;
pub mod webserver;
