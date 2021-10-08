use reqwest::header::HeaderValue;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};

pub struct GitHubService {
    client: Client,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DirectoryEntryType {
    #[serde(rename = "dir")]
    Directory,
    #[serde(rename = "file")]
    File,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub download_url: Option<String>,
    pub sha: String,
    pub r#type: DirectoryEntryType,
}

impl GitHubService {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .user_agent(HeaderValue::from_static("reqwest 0.11.5"))
            .build()
            .expect("Unable to create HTTP client for GitHub Service");

        GitHubService { client }
    }

    pub async fn repo_path_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Vec<DirectoryEntry> {
        let uri = format!(
            "https://api.github.com/repos/{owner}/{repo}/contents/{path}",
            owner = owner,
            repo = repo,
            path = path
        );

        let response = self
            .client
            .get(uri)
            .send()
            .await
            .unwrap()
            .json::<Vec<DirectoryEntry>>()
            .await
            .unwrap();

        response
    }
}
