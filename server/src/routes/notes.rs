use rocket::serde::json::Json;
use rocket::State;

use crate::error::Error;
use crate::services::notes::{Metadata, Note};
use crate::services::Services;

#[get("/notes")]
pub async fn index(services: &State<Services>) -> Result<Json<Vec<Metadata>>, Error> {
    let metadata_list = services.notes_service.list().await?;

    Ok(Json(metadata_list))
}

#[get("/notes/<slug>")]
pub async fn find_by_slug(services: &State<Services>, slug: String) -> Result<Json<Note>, Error> {
    let note = services.notes_service.find_by_slug(&slug).await?;

    Ok(Json(note))
}
