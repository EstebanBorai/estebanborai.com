mod routes;
mod services;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(services::Services::new())
        .mount("/hello", routes![routes::index])
}
