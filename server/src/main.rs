mod error;
mod routes;
mod services;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    env_logger::init();

    if cfg!(debug_assertions) {
        dotenv().ok().expect("Failed to load dotenv");
    }

    let services = services::Services::new().await;

    rocket::build().manage(services).mount(
        "/api/v1",
        routes![routes::notes::index, routes::notes::find_by_slug],
    )
}
