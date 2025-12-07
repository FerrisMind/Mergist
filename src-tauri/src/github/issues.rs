use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use crate::github::client::GitHubClient;
use crate::models::{DomainError, Issue};

#[derive(Debug, Deserialize)]
struct IssueUser {
    login: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IssueLabel {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IssueMilestone {
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IssueResponse {
    number: u64,
    title: String,
    state: String,
    user: Option<IssueUser>,
    created_at: Option<String>,
    updated_at: Option<String>,
    closed_at: Option<String>,
    comments: Option<u64>,
    labels: Option<Vec<IssueLabel>>,
    milestone: Option<IssueMilestone>,
    assignees: Option<Vec<IssueUser>>,
    body: Option<String>,
    html_url: Option<String>,
    pull_request: Option<serde_json::Value>,
}

pub async fn fetch_issues(
    client: &GitHubClient,
    owner: &str,
    repo: &str,
    cancel: &CancellationToken,
) -> Result<(Vec<Issue>, bool), DomainError> {
    const PER_PAGE: u32 = 100;
    const MAX_PAGES: u32 = 10; // до 1000 issues
    let mut all = Vec::new();
    let mut truncated = false;

    for page in 1..=MAX_PAGES {
        if cancel.is_cancelled() {
            return Err(DomainError::Cancelled);
        }

        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/issues?state=all&per_page={PER_PAGE}&page={page}"
        );

        let resp = client
            .client()
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound);
        }

        if !resp.status().is_success() {
            return Err(DomainError::Network(format!(
                "GitHub Issues API returned {}",
                resp.status()
            )));
        }

        let page_items: Vec<IssueResponse> = resp
            .json()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        if page_items.is_empty() {
            break;
        }

        for item in &page_items {
            // пропускаем PR
            if item.pull_request.is_some() {
                continue;
            }
            let labels = item
                .labels
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|l| l.name.clone())
                .collect();
            let assignees = item
                .assignees
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|u| u.login.clone())
                .collect();

            let issue = Issue {
                number: item.number,
                title: item.title.clone(),
                state: item.state.clone(),
                author: item.user.as_ref().and_then(|u| u.login.clone()),
                created_at: item.created_at.clone(),
                updated_at: item.updated_at.clone(),
                closed_at: item.closed_at.clone(),
                comments: item.comments,
                labels,
                milestone: item.milestone.as_ref().and_then(|m| m.title.clone()),
                assignees,
                body: item.body.clone(),
                html_url: item.html_url.clone(),
            };
            all.push(issue);
        }

        if all.len() as u32 >= PER_PAGE * page {
            // continue if page full
        }

        if page_items.len() < PER_PAGE as usize {
            break;
        }

        if page == MAX_PAGES {
            truncated = true;
        }
    }

    Ok((all, truncated))
}
