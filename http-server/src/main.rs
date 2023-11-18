#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod http;
mod server;
mod website_handler;

use server::Server;
use std::{default, env, fmt::format};
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path {}", public_path);
    let server = Server::new(String::from("127.0.0.1:8080").to_string());

    server.run(WebsiteHandler::new(public_path));
}
