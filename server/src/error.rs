use reqwest::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use serde_json::to_string;
use std::io::Cursor;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub struct Error {
    message: String,
    status_code: u16,
}

impl Error {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        Error {
            message: message.to_string(),
            status_code: status_code.as_u16(),
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        log::error!("{:#?}", err);

        if err.is_status() {
            return Error::new(err.status().unwrap(), &err.to_string());
        }

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unhandled error ocurred performing a HTTP request",
        )
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        log::error!("{:#?}", err);

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unhandled error ocurred",
        )
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(err: sqlx::error::Error) -> Self {
        log::error!("{:#?}", err);

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error ocurred connecting to the database",
        )
    }
}
