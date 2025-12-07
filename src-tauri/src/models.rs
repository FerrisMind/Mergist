use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoInput {
    pub repo: String,
    #[serde(default)]
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoInfo {
    pub owner: String,
    pub repo: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub subdirectory: Option<String>,
    pub original_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOptions {
    #[serde(default = "default_true")]
    pub include_filenames: bool,
    #[serde(default = "default_true")]
    pub add_separators: bool,
    #[serde(default = "default_true")]
    pub skip_large_files: bool,
    #[serde(default = "default_true")]
    pub remove_license_headers: bool,
    #[serde(default)]
    pub skip_patterns: Vec<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stats {
    pub files_processed: u64,
    pub files_skipped: u64,
    pub total_size_bytes: u64,
    pub total_lines: u64,
    pub token_count: Option<u64>,
    pub total_files: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub file_path: String,
    pub stats: Stats,
    pub repo: RepoInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesExportOptions {
    #[serde(default = "default_true")]
    pub include_open: bool,
    #[serde(default)]
    pub include_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub state: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub closed_at: Option<String>,
    #[serde(default)]
    pub comments: Option<u64>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub milestone: Option<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuesStats {
    pub total: u64,
    pub open: u64,
    pub closed: u64,
    #[serde(default)]
    pub latest_updated: Option<String>,
    #[serde(default)]
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesExportResult {
    pub file_path: String,
    pub repo: RepoInfo,
    pub stats: IssuesStats,
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid repository format: {0}")]
    InvalidRepo(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Repository not found or inaccessible")]
    NotFound,
    #[error("No files to process after filtering")]
    NoFiles,
    #[error("Operation cancelled")]
    Cancelled,
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
