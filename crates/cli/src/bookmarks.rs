use std::collections::HashSet;
use std::env::current_dir;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use chrono::{DateTime, Utc};
use link_preview::LinkPreview;
use serde::{Deserialize, Serialize};

const BOOKMARKS_INDEX_FILE: &str = "bookmarks.json";

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkEntry {
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub preview_image_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Hash for BookmarkEntry {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let _ = &self.url.hash(state);
    }
}

impl PartialEq for BookmarkEntry {
    fn eq(&self, other: &Self) -> bool {
        &self.url == &other.url
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarksIndex(HashSet<BookmarkEntry>);

impl BookmarksIndex {
    /// Opens the BookmarksIndex from the `bookmarks.json` file.
    pub fn new() -> Result<Self> {
        let path = current_dir()?
            .join("static")
            .join("db")
            .join(BOOKMARKS_INDEX_FILE);
        let index = read_to_string(path)?;

        Ok(serde_json::from_str(&index)?)
    }

    /// Lists entries
    pub fn list(&self) {
        self.0.iter().for_each(|entry| {
            println!("{}: {}", entry.title, entry.url);
        });
    }

    fn save(&self) -> Result<()> {
        let path = current_dir()?
            .join("static")
            .join("db")
            .join(BOOKMARKS_INDEX_FILE);
        let mut items = self.0
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let index = serde_json::to_string_pretty(&items)?;

        std::fs::write(path, index)?;

        Ok(())
    }

    pub async fn append(&mut self, url: String) -> Result<()> {
        let html = link_preview::fetch::fetch_partially(&url).await?;
        let preview = LinkPreview::from(html);
        let entry = BookmarkEntry {
            title: preview.title.unwrap_or(url.to_owned()),
            url,
            tags: Vec::new(),
            description: preview.description,
            preview_image_url: preview.image_url.map(|url| url.to_string()),
            created_at: Utc::now(),
        };

        self.0.insert(entry.to_owned());
        self.save()?;

        println!("Added Bookmark Entry: {:#?}", entry);

        Ok(())
    }
}
