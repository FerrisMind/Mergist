use std::fs;
use std::io::Write;
use std::path::PathBuf;

use tempfile::{NamedTempFile, PersistError};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use tokio_util::sync::CancellationToken;

use crate::converter::files::{SkipMatcher, filter_files};
use crate::converter::tree::generate_directory_tree;
use crate::github::client::GitHubClient;
use crate::github::parser::parse_repository_input;
use crate::models::{ConversionResult, ConvertOptions, DomainError, FileEntry, RepoInfo, Stats};

const SEPARATOR: &str =
    "================================================================================";

pub fn remove_license_headers(content: &str) -> String {
    let license_keywords = ["license", "copyright", "mit", "apache", "gpl", "bsd"];
    let lines: Vec<&str> = content.lines().collect();
    let mut start_index = 0usize;
    let max_scan = 50usize;

    let is_comment_like = |s: &str| {
        let t = s.trim_start();
        t.is_empty()
            || t.starts_with("//")
            || t.starts_with('#')
            || t.starts_with("/*")
            || t.starts_with('*')
            || t.starts_with("<!--")
    };

    for (i, line) in lines.iter().take(max_scan).enumerate() {
        let lower = line.to_lowercase();
        if license_keywords.iter().any(|k| lower.contains(k)) && is_comment_like(line) {
            start_index = i + 1;
            continue;
        }

        if is_comment_like(line) {
            start_index = i + 1;
            continue;
        }

        break;
    }

    let mut cleaned = lines[start_index..].join("\n");
    while cleaned.starts_with('\n') {
        cleaned.remove(0);
    }
    cleaned
}

pub async fn resolve_repository_files(
    client: &GitHubClient,
    input: &RepoInfo,
    cancel: &CancellationToken,
) -> Result<(RepoInfo, Vec<FileEntry>), DomainError> {
    let mut branches = Vec::new();
    if let Some(branch) = &input.branch {
        branches.push(branch.clone());
    }
    branches.extend(
        ["main", "master", "dev", "develop"]
            .iter()
            .map(|s| s.to_string()),
    );

    for branch in branches {
        let tree = client
            .get_tree(&input.owner, &input.repo, &branch, cancel)
            .await;
        match tree {
            Ok(files) => {
                let mut info = input.clone();
                info.branch = Some(branch);
                return Ok((info, files));
            }
            Err(DomainError::Cancelled) => return Err(DomainError::Cancelled),
            Err(DomainError::NotFound) => continue,
            Err(e) => return Err(e),
        }
    }

    Err(DomainError::NotFound)
}

pub async fn convert_repository_to_markdown(
    client: &GitHubClient,
    repo_input: &str,
    options: &ConvertOptions,
    output_path: Option<&str>,
    cancel: &CancellationToken,
    mut on_progress: impl FnMut(u64, u64),
) -> Result<ConversionResult, DomainError> {
    let repo_info = parse_repository_input(repo_input)?;
    let (repo_info, files) = resolve_repository_files(client, &repo_info, cancel).await?;

    let matcher = SkipMatcher::new(&options.skip_patterns);
    let filtered: Vec<&FileEntry> = filter_files(&files, &matcher, options);

    if filtered.is_empty() {
        return Err(DomainError::NoFiles);
    }

    let repo_path = match &repo_info.subdirectory {
        Some(sub) if !sub.is_empty() => format!("{}/{}/{}", repo_info.owner, repo_info.repo, sub),
        _ => format!("{}/{}", repo_info.owner, repo_info.repo),
    };

    // Дерево каталога строим по тем файлам, что реально пойдут в экспорт
    let tree = generate_directory_tree(&filtered, repo_info.subdirectory.as_deref());

    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let timestamp = now.format(&Rfc3339).unwrap_or_else(|_| "unknown".into());

    let branch = repo_info.branch.clone().unwrap_or_else(|| "unknown".into());
    // Количество файлов, реально попавших в экспорт (после фильтрации)
    let total_files = filtered.len();

    let header = format!(
        "This document contains the complete source code of the repository consolidated into a single file for streamlined AI analysis.\n\
The repository contents have been processed and combined with security validation bypassed.\n\
\n\
# Repository Overview\n\
\n\
## About This Document\n\
This consolidated file represents the complete codebase from the repository, \n\
merged into a unified document optimized for AI consumption and automated \n\
analysis workflows.\n\
\n\
## Repository Information\n\
- **Repository:** {repo_path}\n\
- **Branch:** {branch}\n\
- **Total Files:** {total_files}\n\
- **Generated:** {timestamp}\n\
\n\
## Document Structure\n\
The content is organized in the following sequence:\n\
1. This overview section\n\
2. Repository metadata and information  \n\
3. File system hierarchy\n\
4. Repository files (when included)\n\
5. Individual source files, each containing:\n\
   a. File path header (## File: path/to/file)\n\
   b. Complete file contents within code blocks   \n\
\n\
## Best Practices\n\
- Treat this document as read-only - make changes in the original repository\n\
- Use file path headers to navigate between different source files\n\
- Handle with appropriate security measures as this may contain sensitive data\n\
- This consolidated view is generated from the live repository state\n\
\n\
## Important Notes\n\
- Files excluded by .gitignore and configuration rules are omitted\n\
- Binary assets are not included - refer to the file structure for complete file listings\n\
- Default ignore patterns have been applied to filter content\n\
- Security validation is disabled - review content for sensitive information carefully\n\
\n\
# Repository Structure\n\
\n\
```\n{repo_path}/\n{tree}```\n\n",
    );

    let mut temp_file = NamedTempFile::new().map_err(|e| DomainError::Io(e.to_string()))?;
    temp_file
        .write_all(header.as_bytes())
        .map_err(|e| DomainError::Io(e.to_string()))?;

    let mut stats = Stats {
        files_processed: 0,
        files_skipped: (files.len() - filtered.len()) as u64,
        total_size_bytes: header.len() as u64,
        total_lines: header.lines().count() as u64,
        token_count: None,
        total_files: Some(filtered.len() as u64),
    };

    for (idx, file) in filtered.iter().enumerate() {
        if cancel.is_cancelled() {
            return Err(DomainError::Cancelled);
        }

        let content = client
            .get_raw_content(
                &repo_info.owner,
                &repo_info.repo,
                repo_info.branch.as_deref().unwrap_or("main"),
                &file.path,
                cancel,
            )
            .await?;

        let mut processed = if options.remove_license_headers {
            remove_license_headers(&content)
        } else {
            content
        };

        if options.add_separators {
            temp_file
                .write_all(SEPARATOR.as_bytes())
                .and_then(|_| temp_file.write_all(b"\n"))
                .map_err(|e| DomainError::Io(e.to_string()))?;
            stats.total_size_bytes += SEPARATOR.len() as u64 + 1;
            stats.total_lines += 1;
        }

        if options.include_filenames {
            temp_file
                .write_all(format!("// File: {}\n", file.path).as_bytes())
                .map_err(|e| DomainError::Io(e.to_string()))?;
            stats.total_size_bytes += format!("// File: {}\n", file.path).as_bytes().len() as u64;
            stats.total_lines += 1;
            if options.add_separators {
                temp_file
                    .write_all(SEPARATOR.as_bytes())
                    .and_then(|_| temp_file.write_all(b"\n"))
                    .map_err(|e| DomainError::Io(e.to_string()))?;
                stats.total_size_bytes += SEPARATOR.len() as u64 + 1;
                stats.total_lines += 1;
            }
        }

        if !processed.ends_with('\n') {
            processed.push('\n');
        }

        temp_file
            .write_all(processed.as_bytes())
            .map_err(|e| DomainError::Io(e.to_string()))?;
        stats.total_size_bytes += processed.len() as u64;
        stats.total_lines += processed.lines().count() as u64;

        if options.add_separators {
            temp_file
                .write_all(b"\n")
                .map_err(|e| DomainError::Io(e.to_string()))?;
            stats.total_size_bytes += 1;
            stats.total_lines += 1;
        }

        stats.files_processed += 1;

        on_progress(idx as u64 + 1, filtered.len() as u64);
    }

    temp_file
        .flush()
        .map_err(|e| DomainError::Io(e.to_string()))?;

    let final_path = match output_path {
        Some(path) => PathBuf::from(path),
        None => {
            // Формат: owner-repo-YYYY-MM-DDTHH-MM-SS (локальное время)
            let ts_file = format!(
                "{:04}-{:02}-{:02}T{:02}-{:02}-{:02}",
                now.year(),
                now.month() as u8,
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            );
            let filename = format!("{}-{}-{}.md", repo_info.owner, repo_info.repo, ts_file);
            std::env::temp_dir().join(filename)
        }
    };

    // atomic move/copy
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

    Ok(ConversionResult {
        file_path: final_path.to_str().unwrap_or_default().to_string(),
        stats: Stats {
            total_files: Some(files.len() as u64),
            ..stats
        },
        repo: repo_info,
    })
}
