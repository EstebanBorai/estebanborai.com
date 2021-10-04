use rocket::State;

use crate::services::Services;

#[get("/<name>")]
pub fn index(state: &State<Services>, name: String) -> String {
    format!("Hello {} and !", name)
}
