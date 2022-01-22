use chrono::{NaiveDate, Utc};
use diesel::query_dsl::QueryDsl;
use diesel::ExpressionMethods;
use diesel::{OptionalExtension, RunQueryDsl};
use reqwest::header::HeaderValue;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::cmp::Ordering;
use std::sync::Arc;
use uuid::Uuid;
use yaml_front_matter::{Document, YamlFrontMatter};

use crate::error::Result;
use crate::models::Note;
use crate::schema;
use crate::services::github::{DirectoryEntryType, GitHubService};
use crate::services::PgConnPool;

/// Metadata extracted from a note's YAML front matter section
#[derive(Deserialize, Serialize)]
pub struct MarkdownFileYamlFrontMatter {
    title: String,
    description: String,
    categories: Vec<String>,
    date: String,
    lang: String,
    preview_image_url: String,
}

pub struct NotesService {
    client: Client,
    github_service: Arc<GitHubService>,
    dbconn_pool: Arc<PgConnPool>,
}

impl NotesService {
    pub fn new(github_service: Arc<GitHubService>, dbconn_pool: Arc<PgConnPool>) -> Self {
        let client = ClientBuilder::new()
            .user_agent(HeaderValue::from_static("reqwest v0.11.5"))
            .build()
            .expect("Failed to build HTTP Client for Notes Service");

        Self {
            client,
            dbconn_pool,
            github_service,
        }
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Note>> {
        let dbconn = self.dbconn_pool.get().unwrap();
        let result = schema::notes::table
            .filter(schema::notes::slug.eq(slug))
            .first::<Note>(&dbconn);

        Ok(result.optional()?)
    }

    /// Lists all Markdown file's metadata living under the "notes" directory in
    /// the EstebanBorai/EstebanBorai repository
    pub async fn list(&self) -> Result<Vec<Note>> {
        let contents = self
            .github_service
            .repo_path_contents("EstebanBorai", "EstebanBorai", "notes")
            .await;
        let mut notes: Vec<Note> = Vec::new();

        for dir_entry in contents {
            if dir_entry.r#type == DirectoryEntryType::File {
                let download_url = dir_entry.download_url.unwrap();
                let res = self.client.get(&download_url).send().await?;
                let markdown = res.text().await.unwrap();
                let Document {
                    content,
                    metadata: yfm,
                } = YamlFrontMatter::parse::<MarkdownFileYamlFrontMatter>(&markdown).unwrap();
                let now = Utc::now().naive_utc();

                notes.push(Note {
                    id: Uuid::new_v4(),
                    slug: slugify(&yfm.title),
                    description: yfm.description,
                    categories: yfm.categories,
                    title: yfm.title,
                    date: NaiveDate::parse_from_str(&yfm.date, "%Y-%m-%d").unwrap(),
                    sha: dir_entry.sha,
                    lang: yfm.lang,
                    preview_image_url: yfm.preview_image_url,
                    download_url,
                    content,
                    created_at: now,
                    updated_at: now,
                });
            }

            continue;
        }

        let dbconn = self.dbconn_pool.get().unwrap();

        diesel::insert_into(schema::notes::table)
            .values(&notes)
            .on_conflict_do_nothing()
            .execute(&dbconn)
            .unwrap();

        notes.sort_by(|a, b| {
            if b.date > a.date {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        Ok(notes)
    }
}
