use std::sync::Arc;

use reqwest::{Client, header};
use tokio_util::sync::CancellationToken;

use crate::models::{DomainError, FileEntry};

const USER_AGENT: &str = "mergist";

#[derive(Clone)]
pub struct GitHubClient {
    client: Arc<Client>,
}

impl GitHubClient {
    pub fn new() -> Result<Self, DomainError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub fn client(&self) -> &Client {
        self.client.as_ref()
    }

    pub async fn get_tree(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        cancel: &CancellationToken,
    ) -> Result<Vec<FileEntry>, DomainError> {
        if cancel.is_cancelled() {
            return Err(DomainError::Cancelled);
        }

        let url =
            format!("https://api.github.com/repos/{owner}/{repo}/git/trees/{branch}?recursive=1");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound);
        }

        if !response.status().is_success() {
            return Err(DomainError::Network(format!(
                "GitHub Trees API returned {}",
                response.status()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let Some(tree_items) = data.get("tree").and_then(|t| t.as_array()) else {
            return Err(DomainError::Unexpected(
                "Malformed GitHub Trees response".into(),
            ));
        };

        let mut files = Vec::new();
        for item in tree_items {
            if item
                .get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "blob")
                .unwrap_or(false)
            {
                if cancel.is_cancelled() {
                    return Err(DomainError::Cancelled);
                }

                let path = item
                    .get("path")
                    .and_then(|p| p.as_str())
                    .unwrap_or_default()
                    .to_string();

                let size = item.get("size").and_then(|s| s.as_u64()).unwrap_or(0);

                files.push(FileEntry { path, size });
            }
        }

        if files.is_empty() {
            return Err(DomainError::NotFound);
        }

        Ok(files)
    }

    pub async fn get_raw_content(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        path: &str,
        cancel: &CancellationToken,
    ) -> Result<String, DomainError> {
        if cancel.is_cancelled() {
            return Err(DomainError::Cancelled);
        }

        let url = format!("https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        if cancel.is_cancelled() {
            return Err(DomainError::Cancelled);
        }

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound);
        }

        if !response.status().is_success() {
            return Err(DomainError::Network(format!(
                "Raw content returned {}",
                response.status()
            )));
        }

        response
            .text()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }
}
