use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::services::notes::Metadata;
use crate::services::Services;

#[get("/notes")]
pub async fn index(services: &State<Services>) -> (Status, Json<Vec<Metadata>>) {
    let metadata_list = services.notes_service.list().await;

    (Status::Ok, Json(metadata_list))
}
