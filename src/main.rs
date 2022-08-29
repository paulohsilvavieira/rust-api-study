mod controllers;
mod infra;
mod services;
use infra::database::*;

fn main() {
    connection::establish();
    println!("Hello, world!");
}
