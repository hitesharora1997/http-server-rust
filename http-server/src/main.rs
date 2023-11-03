fn main() {
    let server = Server::new(String::from("127.0.0.1:8080").to_string());

    // dbg!(server);

    server.run();
    // dbg!(server);

    // let s = String::from("value");
    // let j: &str = &s;

    // let string_borrow: &str = &Server { addr: val };
}

#[derive(Debug)]
struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    fn run(self) {
        println!("Running the server on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: String,
}
