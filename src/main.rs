#[macro_use]
extern crate rocket;

mod can;
mod caniot;
mod config;
mod shared;
mod controller;
mod init;
mod server;
mod shutdown;
mod logger;

fn main() {
    init::init_controller();
}
