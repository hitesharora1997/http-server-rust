use crate::http::Response;

use super::server::Handler;
pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> Response {
        Response::new(
            crate::http::StatusCode::Ok,
            Some("<h1>TEST</h1>".to_string()),
        )
    }
}
