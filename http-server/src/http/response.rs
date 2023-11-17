use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum StatusCode {}

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) {
        unimplemented!()
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(f, "HTTP/1.1 \r\n\r\n {:?}", self.status_code,)
    }
}
