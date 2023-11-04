pub mod request {
    #[derive(Debug)]
    struct Request {
        path: String,
        query_string: Option<String>,
        method: super::method::Method,
    }
}
