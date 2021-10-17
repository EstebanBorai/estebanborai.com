mod config;
mod error;
mod routes;
mod services;

use std::env;

use self::config::Config;

#[macro_use]
extern crate log;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    if cfg!(debug_assertions) {
        dotenv().ok().expect("Failed to load dotenv");
    }

    let config = Config::new();
    env_logger::init();

    let services = services::Services::new(&config).await;

    rocket::custom(&config.server_config)
        .manage(services)
        .mount(
            "/api/v1",
            routes![routes::notes::index, routes::notes::find_by_slug],
        )
}
