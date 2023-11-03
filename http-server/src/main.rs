use http::request::Request;
use server::Server;

fn main() {
    let _get = http::method::Method::GET("hello".to_string());
    // let _post = Method::POST;
    // let _patch = Method::PATCH;
    // let _put = Method::PUT;
    // let _head = Method::HEAD;
    // let _connect = Method::CONNECT;
    // let _delete = Method::DELETE(100);
    let server = server::Server::new(String::from("127.0.0.1:8080").to_string());

    // dbg!(server);

    server.run();
    // dbg!(server);

    // let s = String::from("value");
    // let j: &str = &s;

    // let string_borrow: &str = &Server { addr: val };
}

mod server {

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
        }
    }
}

mod http {

    pub mod request {
        // #[derive(Debug)]
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }

    pub mod method {

        // #[derive(Debug)]
        pub enum Method {
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
    }
}
