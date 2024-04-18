// TODO: Disable this for production
// #![allow(dead_code, unused_imports)]

#[macro_use]
extern crate rocket;

mod bus;
mod caniot;
mod config;
mod controller;
mod database;
mod init;
mod logger;
mod shared;
mod shutdown;
mod webserver;

#[cfg(feature = "grpc")]
mod grpcserver;

fn main() {
    init::run_controller();
}
