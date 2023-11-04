pub mod method {

    #[derive(Debug)]
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
