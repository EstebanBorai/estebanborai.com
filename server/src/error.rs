use reqwest::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use serde_json::to_string;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
    status_code: u16,
}

impl ErrorResponse {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        ErrorResponse {
            message: message.to_string(),
            status_code: status_code.as_u16(),
        }
    }
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let status = Status::from_code(self.status_code).unwrap();
        let body = to_string(&self).unwrap();

        Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}
