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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        let err = Error {
            message: message.to_string(),
            status_code: status_code.as_u16(),
        };

        if cfg!(not(debug_assertions)) {
            sentry::capture_error(&err);
        }

        err
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
        error!("{:#?}", err);
        if cfg!(not(debug_assertions)) {
            sentry::capture_error(&err);
        }

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
        error!("{:#?}", err);

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unhandled error ocurred",
        )
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        use diesel::result::Error as DieselError;

        match err {
            DieselError::NotFound => Error::new(StatusCode::OK, "Resource not found"),
            _ => Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database error ocurred",
            ),
        }
    }
}
