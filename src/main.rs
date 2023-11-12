// TODO: Disable this for production
// #![allow(dead_code, unused_imports)]

#[macro_use]
extern crate rocket;

mod can;
mod caniot;
mod config;
mod controller;
mod grpcserver;
mod init;
mod logger;
mod shared;
mod shutdown;
mod webserver;
// mod devices;

fn main() {
    init::run_controller();
}
