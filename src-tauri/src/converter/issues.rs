use std::fs;
use std::io::Write;
use std::path::PathBuf;

use tempfile::{NamedTempFile, PersistError};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use tokio_util::sync::CancellationToken;

use crate::github::client::GitHubClient;
use crate::github::issues::fetch_issues;
use crate::models::{
    DomainError, Issue, IssuesExportOptions, IssuesExportResult, IssuesStats, RepoInfo,
};

pub fn filter_issues<'a>(issues: &'a [Issue], opts: &IssuesExportOptions) -> Vec<&'a Issue> {
    issues
        .iter()
        .filter(|issue| match issue.state.as_str() {
            "open" => opts.include_open,
            "closed" => opts.include_closed,
            _ => false,
        })
        .collect()
}

fn generate_issues_markdown(
    repo: &RepoInfo,
    issues: &[&Issue],
    truncated: bool,
    opts: &IssuesExportOptions,
) -> String {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let generated = now.format(&Rfc3339).unwrap_or_else(|_| "unknown".into());

    let open_count = issues.iter().filter(|i| i.state == "open").count();
    let closed_count = issues.iter().filter(|i| i.state == "closed").count();

    let included_states = match (opts.include_open, opts.include_closed) {
        (true, true) => "Open, Closed",
        (true, false) => "Open",
        (false, true) => "Closed",
        _ => "None",
    };

    let mut content = format!(
        "# Issues Export for {}/{}\n\n- **Generated:** {generated}\n- **Total Issues:** {}\n- **Open Issues:** {}\n- **Closed Issues:** {}\n- **Included States:** {}\n\nThis document consolidates GitHub issues into a single Markdown file for review, backups, or AI ingestion.\n",
        repo.owner,
        repo.repo,
        issues.len(),
        open_count,
        closed_count,
        included_states
    );

    if truncated {
        content.push_str(&format!(
            "\n> ⚠️ Only the first {} matching issues are included due to API pagination limits.\n",
            issues.len()
        ));
    }

    if issues.is_empty() {
        content.push_str("\nNo issues were found for this repository with the current filters.\n");
        return content;
    }

    content.push_str("\n---\n");

    for issue in issues {
        let labels = if issue.labels.is_empty() {
            "None".to_string()
        } else {
            issue.labels.join(", ")
        };
        let milestone = issue.milestone.clone().unwrap_or_else(|| "None".into());
        let assignees = if issue.assignees.is_empty() {
            "None".to_string()
        } else {
            issue.assignees.join(", ")
        };
        let comments = issue.comments.unwrap_or(0);

        content.push_str(&format!(
      "\n## #{}: {}\n\n- **State:** {}\n- **Author:** {}\n- **Created:** {}\n- **Updated:** {}\n- **Closed:** {}\n- **Comments:** {}\n- **Labels:** {}\n- **Milestone:** {}\n- **Assignees:** {}\n- **URL:** {}\n",
      issue.number,
      issue.title,
      issue.state.to_uppercase(),
      issue.author.clone().unwrap_or_else(|| "Unknown".into()),
      issue.created_at.clone().unwrap_or_else(|| "N/A".into()),
      issue.updated_at.clone().unwrap_or_else(|| "N/A".into()),
      issue.closed_at.clone().unwrap_or_else(|| "N/A".into()),
      comments,
      labels,
      milestone,
      assignees,
      issue.html_url.clone().unwrap_or_else(|| "N/A".into())
    ));

        if let Some(body) = &issue.body {
            if !body.trim().is_empty() {
                content.push_str("\n### Description\n\n");
                content.push_str(body);
                content.push('\n');
            } else {
                content.push_str("\n_No description provided._\n");
            }
        } else {
            content.push_str("\n_No description provided._\n");
        }

        content.push_str("\n---\n");
    }

    content
}

pub async fn export_issues_to_markdown(
    client: &GitHubClient,
    repo_info: &RepoInfo,
    options: &IssuesExportOptions,
    output_path: Option<&str>,
    cancel: &CancellationToken,
    mut on_progress: impl FnMut(u64, u64),
) -> Result<IssuesExportResult, DomainError> {
    let (all_issues, truncated) =
        fetch_issues(client, &repo_info.owner, &repo_info.repo, cancel).await?;

    let filtered = filter_issues(&all_issues, options);
    let total = filtered.len() as u64;
    let open = filtered.iter().filter(|i| i.state == "open").count() as u64;
    let closed = filtered.iter().filter(|i| i.state == "closed").count() as u64;

    if filtered.is_empty() {
        return Err(DomainError::NoFiles);
    }

    let latest_updated = filtered
        .iter()
        .filter_map(|i| i.updated_at.as_ref())
        .max()
        .cloned();

    let markdown = generate_issues_markdown(repo_info, &filtered, truncated, options);

    let mut temp_file = NamedTempFile::new().map_err(|e| DomainError::Io(e.to_string()))?;
    temp_file
        .write_all(markdown.as_bytes())
        .map_err(|e| DomainError::Io(e.to_string()))?;
    temp_file
        .flush()
        .map_err(|e| DomainError::Io(e.to_string()))?;

    let final_path = match output_path {
        Some(path) => PathBuf::from(path),
        None => {
            let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
            let ts_file = format!(
                "{:04}-{:02}-{:02}T{:02}-{:02}-{:02}",
                now.year(),
                now.month() as u8,
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            );
            let filename = format!(
                "{}-{}-{}-issues.md",
                repo_info.owner, repo_info.repo, ts_file
            );
            std::env::temp_dir().join(filename)
        }
    };

    if let Err(err) = temp_file.persist(&final_path) {
        let PersistError { file, error } = err;
        let copy_result = fs::copy(file.path(), &final_path);
        if let Err(copy_err) = copy_result {
            return Err(DomainError::Io(format!(
                "persist fallback failed: {}; original: {}",
                copy_err, error
            )));
        }
    }

    on_progress(total, total);

    Ok(IssuesExportResult {
        file_path: final_path.to_string_lossy().into_owned(),
        repo: repo_info.clone(),
        stats: IssuesStats {
            total,
            open,
            closed,
            latest_updated,
            truncated,
        },
    })
}
