use futures::future::join_all;
use reqwest::header::HeaderValue;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use yaml_front_matter::YamlFrontMatter;

use crate::services::github::{DirectoryEntryType, GitHubService};

pub struct NotesService {
    client: Client,
    github_service: Arc<GitHubService>,
}

#[derive(Deserialize, Serialize)]
pub struct Metadata {
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

impl NotesService {
    pub fn new(github_service: Arc<GitHubService>) -> Self {
        let client = ClientBuilder::new()
            .user_agent(HeaderValue::from_static("reqwest 0.11.5"))
            .build()
            .expect("Unable to create HTTP client for GitHub Service");

        NotesService {
            client,
            github_service,
        }
    }

    /// Lists all Markdown file's metadata living under the "notes" directory in
    /// the EstebanBorai/EstebanBorai repository
    pub async fn list(&self) -> Vec<Metadata> {
        // Fetch all directory entries in the EstebanBorai/EstebanBorai
        // repository. Entries under the `notes` directory must be retrieved
        let response = self
            .github_service
            .repo_path_contents("EstebanBorai", "EstebanBorai", "notes")
            .await;

        let metadata_futures = response
            .into_iter()
            .filter(|directory_entry| {
                if directory_entry.name.as_str().ends_with(".md")
                    && matches!(directory_entry.r#type, DirectoryEntryType::File)
                {
                    return true;
                }

                false
            })
            .map(|directory_entry| {
                Box::pin(self.fetch_markdown_metadata(directory_entry.download_url.unwrap()))
            })
            .collect::<Vec<_>>();

        join_all(metadata_futures).await
    }

    async fn fetch_markdown_metadata(&self, markdown_url: String) -> Metadata {
        let response = self
            .client
            .get(&markdown_url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        YamlFrontMatter::parse::<Metadata>(&response)
            .unwrap()
            .metadata
    }
}
