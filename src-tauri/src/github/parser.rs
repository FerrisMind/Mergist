use crate::models::{DomainError, RepoInfo};
use url::Url;

pub fn parse_repository_input(input: &str) -> Result<RepoInfo, DomainError> {
    let normalized = input.trim();

    if normalized.contains("github.com") {
        // Accept inputs without scheme (e.g. github.com/owner/repo) by prefixing https://
        let candidate = if normalized.starts_with("http://") || normalized.starts_with("https://") {
            normalized.to_string()
        } else {
            format!(
                "https://{}",
                normalized
                    .trim_start_matches("https://")
                    .trim_start_matches("http://")
            )
        };

        let url =
            Url::parse(&candidate).map_err(|_| DomainError::InvalidRepo(normalized.to_string()))?;
        let mut segments: Vec<String> = url
            .path_segments()
            .map(|s| s.map(str::to_string).collect())
            .unwrap_or_default();

        segments.retain(|s| !s.is_empty());
        if segments.len() < 2 {
            return Err(DomainError::InvalidRepo(normalized.to_string()));
        }

        let owner = segments[0].clone();
        let repo = segments[1].trim_end_matches(".git").to_string();

        let mut branch: Option<String> = None;
        let mut subdirectory: Option<String> = None;

        if segments.len() > 2 {
            let route = segments[2].as_str();
            match route {
                "tree" | "blob" | "raw" => {
                    if segments.len() > 3 {
                        branch = Some(segments[3].clone());
                    }
                    if segments.len() > 4 {
                        subdirectory = Some(segments[4..].join("/"));
                    }
                }
                other => {
                    let special = ["issues", "pulls", "pull", "actions", "commits", "releases"];
                    if !special.contains(&other) && segments.len() > 2 {
                        subdirectory = Some(segments[2..].join("/"));
                    }
                }
            }
        }

        return Ok(RepoInfo {
            owner,
            repo,
            branch,
            subdirectory,
            original_url: normalized.to_string(),
        });
    }

    let parts: Vec<&str> = normalized.split('/').collect();
    if parts.len() != 2 {
        return Err(DomainError::InvalidRepo(normalized.to_string()));
    }

    Ok(RepoInfo {
        owner: parts[0].to_string(),
        repo: parts[1].to_string(),
        branch: None,
        subdirectory: None,
        original_url: normalized.to_string(),
    })
}
