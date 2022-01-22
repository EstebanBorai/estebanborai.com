use async_graphql::SimpleObject;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::notes;

#[derive(Clone, Debug, Deserialize, Insertable, PartialEq, Serialize, SimpleObject, Queryable)]
#[table_name = "notes"]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub categories: Vec<String>,
    pub date: NaiveDate,
    pub sha: String,
    pub lang: String,
    pub preview_image_url: String,
    pub download_url: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
