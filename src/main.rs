#[macro_use]
extern crate rocket;

mod can;
mod caniot;
mod config;
mod controller;
mod init;
mod logger;
mod webserver;
mod grpcserver;
mod shared;
mod shutdown;

fn main() {
    init::init_controller();
}
