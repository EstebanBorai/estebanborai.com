use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use reqwest::header::HeaderValue;
use reqwest::{Client, ClientBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::postgres::PgPool;
use sqlx::{FromRow, Row};
use std::sync::Arc;
use uuid::Uuid;
use yaml_front_matter::YamlFrontMatter;

use crate::error::{Error, Result};
use crate::services::github::{DirectoryEntryType, GitHubService};

use super::github::DirectoryEntry;

pub struct NotesService {
    client: Client,
    database: Arc<PgPool>,
    github_service: Arc<GitHubService>,
}

/// Note's metadata built from a note's markdown file YAML front matter section
/// with extra logic such as file SHA and slug.
#[derive(Deserialize, Serialize)]
pub struct Metadata {
    id: Uuid,
    title: String,
    slug: String,
    description: String,
    categories: Vec<String>,
    date: DateTime<Utc>,
    sha: String,
    lang: String,
}

impl Metadata {
    pub fn from_yaml_front_matter(yfm: MarkdownFileYamlFrontMatter, sha: String) -> Self {
        let slug = slugify(&yfm.title);

        Metadata {
            id: Uuid::default(),
            title: yfm.title,
            slug,
            description: yfm.description,
            categories: yfm.categories,
            // TODO: Parse date from MD file
            date: Utc::now(),
            sha,
            lang: yfm.lang,
        }
    }
}

/// Metadata extracted from a note's YAML front matter section
#[derive(Deserialize, Serialize)]
pub struct MarkdownFileYamlFrontMatter {
    title: String,
    description: String,
    categories: Vec<String>,
    date: String,
    lang: String,
}

#[derive(Deserialize, Serialize)]
pub struct Note {
    metadata: Metadata,
    content: String,
}

#[derive(FromRow)]
struct NoteMetadataRow {
    id: Uuid,
    title: String,
    slug: String,
    description: String,
    categories: String,
    date: DateTime<Utc>,
    lang: String,
    sha: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<Metadata> for NoteMetadataRow {
    fn from(metadata: Metadata) -> Self {
        NoteMetadataRow {
            id: Uuid::default(),
            title: metadata.title.clone(),
            slug: slugify(&metadata.title),
            description: metadata.description.clone(),
            categories: metadata.categories.join(","),
            date: Utc::now(),
            lang: metadata.lang.clone(),
            sha: metadata.sha,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl From<NoteMetadataRow> for Metadata {
    fn from(row: NoteMetadataRow) -> Self {
        Metadata {
            id: row.id,
            title: row.title,
            description: row.description,
            categories: row.categories.split(',').map(String::from).collect(),
            date: row.date,
            sha: row.sha,
            slug: row.slug,
            lang: row.lang,
        }
    }
}

impl NotesService {
    pub fn new(github_service: Arc<GitHubService>, database: Arc<PgPool>) -> Self {
        let client = ClientBuilder::new()
            .user_agent(HeaderValue::from_static("reqwest 0.11.5"))
            .build()
            .expect("Unable to create HTTP client for GitHub Service");

        NotesService {
            client,
            database,
            github_service,
        }
    }

    /// Lists all Markdown file's metadata living under the "notes" directory in
    /// the EstebanBorai/EstebanBorai repository
    pub async fn list(&self) -> Result<Vec<Metadata>> {
        if self.is_notes_metadata_updated().await? {
            let query = std::include_str!("../../sql/select_notes_metadata.sql");
            let rows: Vec<NoteMetadataRow> =
                sqlx::query_as(query).fetch_all(&*self.database).await?;
            let notes = rows
                .into_iter()
                .map(Metadata::from)
                .collect::<Vec<Metadata>>();

            return Ok(notes);
        }

        self.update_notes_metadata().await
    }

    async fn update_notes_metadata(&self) -> Result<Vec<Metadata>> {
        let directory_entries = self
            .github_service
            .repo_path_contents("EstebanBorai", "EstebanBorai", "notes")
            .await;

        let md_yfm = directory_entries
            .into_iter()
            .filter(|directory_entry| {
                if directory_entry.name.as_str().ends_with(".md")
                    && matches!(directory_entry.r#type, DirectoryEntryType::File)
                {
                    return true;
                }

                false
            })
            .map(|directory_entry| Box::pin(self.fetch_markdown_metadata(directory_entry)))
            .collect::<Vec<_>>();

        let md_yfm = join_all(md_yfm)
            .await
            .into_iter()
            .collect::<Result<Vec<(DirectoryEntry, MarkdownFileYamlFrontMatter, String)>>>()?;

        let notes_metadata = md_yfm
            .into_iter()
            .map(|(entry, meta, content)| {
                let meta = Metadata::from_yaml_front_matter(meta, entry.sha.clone());

                self.upsert_note(entry, meta, content)
            })
            .collect::<Vec<_>>();

        join_all(notes_metadata)
            .await
            .into_iter()
            .collect::<Result<Vec<Metadata>>>()
    }

    /// Fetches a note by the provided `slug`.
    #[async_recursion]
    pub async fn find_by_slug(&self, slug: &str) -> Result<Note> {
        if self.is_notes_metadata_updated().await? {
            let query = std::include_str!("../../sql/select_notes_content_where_slug.sql");
            let row = sqlx::query(query)
                .bind(slug)
                .fetch_one(&*self.database)
                .await?;

            if row.is_empty() {
                return Err(Error::new(
                    StatusCode::NOT_FOUND,
                    &format!("No notes found with slug: {}", slug),
                ));
            }
            // TODO: Refactor this to use an easier to mantain approach
            let categories = row
                .get::<String, usize>(4)
                .split(",")
                .map(String::from)
                .collect::<Vec<String>>();

            return Ok(Note {
                metadata: Metadata {
                    id: row.get::<Uuid, usize>(0),
                    title: row.get::<String, usize>(1),
                    slug: row.get::<String, usize>(2),
                    description: row.get::<String, usize>(3),
                    categories,
                    date: row.get::<DateTime<Utc>, usize>(5),
                    sha: row.get::<String, usize>(6),
                    lang: row.get::<String, usize>(7),
                },
                content: row.get::<String, usize>(8),
            });
        }

        self.update_notes_metadata().await?;
        self.find_by_slug(slug).await
    }

    async fn fetch_markdown_metadata(
        &self,
        directory_entry: DirectoryEntry,
    ) -> Result<(DirectoryEntry, MarkdownFileYamlFrontMatter, String)> {
        let markdown_url = directory_entry.download_url.clone().unwrap();
        let response = self.client.get(&markdown_url).send().await?.text().await?;
        let document = YamlFrontMatter::parse::<MarkdownFileYamlFrontMatter>(&response)?;

        Ok((directory_entry, document.metadata, document.content))
    }

    async fn is_notes_metadata_updated(&self) -> Result<bool> {
        let row = sqlx::query(std::include_str!(
            "../../sql/select_most_recent_row_notes_metadata.sql"
        ))
        .fetch_all(&*self.database)
        .await?;

        if row.is_empty() {
            return Ok(false);
        }

        Ok(true)
    }

    async fn upsert_note(
        &self,
        entry: DirectoryEntry,
        metadata: Metadata,
        content: String,
    ) -> Result<Metadata> {
        let metadata = self.upsert_note_metadata(metadata).await?;
        self.upsert_note_content(&metadata.id, &content).await?;

        Ok(metadata)
    }

    async fn upsert_note_metadata(&self, metadata: Metadata) -> Result<Metadata> {
        let query = std::include_str!("../../sql/upsert_notes_metadata.sql");
        let inserted: NoteMetadataRow = sqlx::query_as(query)
            .bind(metadata.title)
            .bind(metadata.slug)
            .bind(metadata.description)
            .bind(metadata.categories)
            .bind(metadata.date)
            .bind(metadata.lang)
            .bind(metadata.sha)
            .fetch_one(&*self.database)
            .await?;

        Ok(Metadata::from(inserted))
    }

    async fn upsert_note_content(&self, note_id: &Uuid, content: &str) -> Result<()> {
        let query = std::include_str!("../../sql/upsert_notes_content.sql");

        sqlx::query(query)
            .bind(&content)
            .bind(&note_id)
            .fetch_one(&*self.database)
            .await?;

        Ok(())
    }
}
