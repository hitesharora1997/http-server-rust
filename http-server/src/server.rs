use crate::http::Request;
use std::io::Read;
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

                            Request::try_from(&buf[..]).unwrap();
                        }
                        Err(e) => println!("failed to read a connection {}", e),
                    }
                }
                Err(e) => println!("failed to establish connection error {}", e),
            }
        }
    }
}
