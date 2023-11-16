#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod http;
mod server;

fn main() {
    let server = server::Server::new(String::from("127.0.0.1:8080").to_string());

    server.run();
}
