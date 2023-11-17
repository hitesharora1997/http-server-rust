use crate::http::{response, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

#[derive(Debug)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        println!("Running the server on {}", self.addr);

        // Recoverable error
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buf));

                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1> nice work</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("failed to read a connection {}", e),
                    }
                }
                Err(e) => println!("failed to establish connection error {}", e),
            }
        }
    }
}
