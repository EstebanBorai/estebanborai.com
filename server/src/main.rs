mod error;
mod routes;
mod services;

use rocket::Config;
use std::env;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    env_logger::init();

    if cfg!(debug_assertions) {
        dotenv().ok().expect("Failed to load dotenv");
    }

    let port = env::var("PORT")
        .unwrap_or(String::from("7878"))
        .parse::<u16>()
        .expect("Not a valid u16 value for PORT environment variable");
    let config = Config {
        port,
        ..Config::debug_default()
    };
    let services = services::Services::new().await;

    rocket::custom(&config).manage(services).mount(
        "/api/v1",
        routes![routes::notes::index, routes::notes::find_by_slug],
    )
}
