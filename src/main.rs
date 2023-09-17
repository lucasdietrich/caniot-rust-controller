#[macro_use] extern crate rocket;

mod can;
mod caniot;
mod config;
mod context;
mod controller;
mod server;
mod init;

fn main() {
    init::init_controller();
}
