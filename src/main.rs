// TODO: Disable this for production
// #![allow(dead_code, unused_imports)]

#[macro_use]
extern crate rocket;

mod can;
mod caniot;
mod config;
mod controller;
mod init;
mod logger;
mod shared;
mod shutdown;
mod webserver;
// mod devices;

#[cfg(feature = "grpc")]
mod grpcserver;

fn main() {
    init::run_controller();
}
