#[macro_use]
extern crate rocket;

mod can;
mod caniot;
mod config;
mod context;
mod controller;
mod init;
mod server;

fn main() {
    init::init_controller();
}
