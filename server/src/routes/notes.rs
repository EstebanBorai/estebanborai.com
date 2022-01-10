use rocket::serde::json::Json;
use rocket::State;

use crate::error::Error;
use crate::models::Note;
use crate::services::Services;

#[get("/notes")]
pub async fn index(services: &State<Services>) -> Result<Json<Vec<Note>>, Error> {
    let metadata_list: Vec<Note> = services.notes_service.list().await?;

    Ok(Json(metadata_list))
}
