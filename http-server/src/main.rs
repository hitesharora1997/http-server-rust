fn main() {
    let _get = Method::GET("hello".to_string());
    let _post = Method::POST;
    let _patch = Method::PATCH;
    let _put = Method::PUT;
    let _head = Method::HEAD;
    let _connect = Method::CONNECT;
    let _delete = Method::DELETE(100);
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
// #[derive(Debug)]
struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
// #[derive(Debug)]
enum Method {
    GET(String),
    POST,
    PUT,
    DELETE(u64),
    CONNECT,
    OPTIONS,
    HEAD,
    TRACE,
    PATCH,
}
