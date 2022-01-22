use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::{Request, Response};
use std::io::Cursor;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let access_control_allow_origin = if cfg!(debug_assertions) {
            "*"
        } else {
            "https://estebanborai.com"
        };

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            access_control_allow_origin,
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(0, Cursor::new(""));
        }
    }
}
