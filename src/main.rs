// TODO: Disable this for production
// #![allow(dead_code, unused_imports)]

#[macro_use]
extern crate rocket;

mod bus;
mod caniot;
mod config;
mod controller;
mod coprocessor;
mod database;
mod grpcserver;
mod init;
mod internal;
mod logger;
mod shared;
mod shutdown;
mod utils;
mod webserver;

fn main() {
    init::run_controller();
}
