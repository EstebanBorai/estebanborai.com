mod config;
mod error;
mod middleware;
mod models;
mod routes;
mod schema;
mod services;

use std::env;

use self::config::Config;

#[macro_use]
extern crate log;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    if cfg!(debug_assertions) {
        dotenv().ok().expect("Failed to load dotenv");
    }

    if cfg!(not(debug_assertions)) {
        let _guard = sentry::init((
            "https://a5eec1eb178d4b368e4dfad2c4b3c044@o446883.ingest.sentry.io/5934543",
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));

        env::set_var("RUST_BACKTRACE", "1");
    }

    let config = Config::new();
    env_logger::init();

    let services = services::Services::new(&config).await;

    rocket::custom(&config.server_config)
        .attach(middleware::cors::Cors)
        .manage(services)
        .mount("/api/v1", routes![routes::notes::index])
}
