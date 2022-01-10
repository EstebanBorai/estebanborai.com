use http_auth_basic::Credentials;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};

pub struct GitHubService {
    client: Client,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum DirectoryEntryType {
    #[serde(rename = "dir")]
    Directory,
    #[serde(rename = "file")]
    File,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub download_url: Option<String>,
    pub sha: String,
    pub r#type: DirectoryEntryType,
}

impl GitHubService {
    pub fn new(github_api_user: &str, github_api_token: &str) -> Self {
        let credentials = Credentials::new(github_api_user, github_api_token).as_http_header();
        let credentials = HeaderValue::from_str(&credentials).unwrap();
        let mut headers = HeaderMap::new();

        headers.insert(HeaderName::from_static("authorization"), credentials);

        let client = ClientBuilder::new()
            .user_agent(HeaderValue::from_static("reqwest v0.11.5"))
            .default_headers(headers)
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
