use crate::http::{Method, Response};

use super::server::Handler;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    crate::http::StatusCode::Ok,
                    Some("<h1>Welcome</h1>".to_string()),
                ),
                "/hello" => Response::new(
                    crate::http::StatusCode::Ok,
                    Some("<h1>hello</h1>".to_string()),
                ),
                _ => Response::new(crate::http::StatusCode::NotFound, None),
            },
            _ => Response::new(crate::http::StatusCode::NotFound, None),
        }
    }
}
