#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod http;
mod server;
mod website_handler;

use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080").to_string());

    server.run(WebsiteHandler);
}
