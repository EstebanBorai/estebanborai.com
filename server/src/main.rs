mod config;
mod error;
mod graphql;
mod middleware;
mod models;
mod routes;
mod schema;
mod services;

use async_graphql::{EmptyMutation, EmptySubscription};
use std::env;
use std::sync::Arc;

use self::config::Config;
use self::graphql::query::Query;
use self::graphql::Schema;
use self::routes::{graphql_playground, graphql_query, graphql_request, preflight};

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
    let services = Arc::new(services);
    let graphql_schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(Arc::clone(&services))
        .finish();

    rocket::custom(&config.server_config)
        .attach(middleware::cors::Cors)
        .manage(services)
        .manage(graphql_schema)
        .mount(
            "/",
            routes![
                graphql_playground,
                graphql_query,
                graphql_request,
                preflight
            ],
        )
}
