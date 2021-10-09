use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::error::ErrorResponse;
use crate::services::notes::Metadata;
use crate::services::Services;

#[get("/notes")]
pub async fn index(services: &State<Services>) -> Result<Json<Vec<Metadata>>, ErrorResponse> {
    let metadata_list = services.notes_service.list().await?;

    Ok(Json(metadata_list))
}
