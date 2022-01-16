use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, response: &mut Response<'r>) {
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
            "GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
