use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NoteMetadata {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub categories: Vec<String>,
    pub date: DateTime<Utc>,
    pub sha: String,
    pub lang: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NoteContent {
    pub metadata: NoteMetadata,
    pub content: String,
}
