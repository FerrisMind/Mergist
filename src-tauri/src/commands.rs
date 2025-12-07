use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;

use tauri::{AppHandle, Emitter, State};
use tokio_util::sync::CancellationToken;

use crate::converter::issues::export_issues_to_markdown;
use crate::converter::processor::convert_repository_to_markdown;
use crate::github::client::GitHubClient;
use crate::github::parser::parse_repository_input;
use crate::models::{
    ConversionResult, ConvertOptions, DomainError, IssuesExportOptions, IssuesExportResult,
    RepoInput,
};

#[derive(Default)]
pub struct ConversionState {
    cancel_token: Mutex<Option<CancellationToken>>,
}

impl ConversionState {
    pub fn set_new(&self) -> CancellationToken {
        let token = CancellationToken::new();
        let mut guard = self.cancel_token.lock().expect("poisoned mutex");
        *guard = Some(token.clone());
        token
    }

    pub fn cancel(&self) {
        if let Some(token) = self.cancel_token.lock().ok().and_then(|mut g| g.take()) {
            token.cancel();
        }
    }
}

fn map_error(err: DomainError) -> String {
    err.to_string()
}

#[tauri::command]
pub async fn convert_repo_to_markdown(
    app: AppHandle,
    state: State<'_, ConversionState>,
    input: RepoInput,
    options: ConvertOptions,
) -> Result<ConversionResult, String> {
    let cancel = state.set_new();
    let client = GitHubClient::new().map_err(map_error)?;

    let progress_emitter = |current: u64, total: u64| {
        let _ = app.emit(
            "conversion-progress",
            serde_json::json!({
              "current": current,
              "total": total,
            }),
        );
    };

    convert_repository_to_markdown(
        &client,
        &input.repo,
        &options,
        input.output_path.as_deref(),
        &cancel,
        progress_emitter,
    )
    .await
    .map_err(map_error)
}

#[tauri::command]
pub async fn export_issues(
    app: AppHandle,
    state: State<'_, ConversionState>,
    input: RepoInput,
    options: IssuesExportOptions,
) -> Result<IssuesExportResult, String> {
    let cancel = state.set_new();
    let client = GitHubClient::new().map_err(map_error)?;
    let repo_info = parse_repository_input(&input.repo).map_err(map_error)?;

    let progress_emitter = |current: u64, total: u64| {
        let _ = app.emit(
            "issues-progress",
            serde_json::json!({ "current": current, "total": total }),
        );
    };

    export_issues_to_markdown(
        &client,
        &repo_info,
        &options,
        input.output_path.as_deref(),
        &cancel,
        progress_emitter,
    )
    .await
    .map_err(map_error)
}

#[tauri::command]
pub async fn cancel_conversion(state: State<'_, ConversionState>) -> Result<(), String> {
    state.cancel();
    Ok(())
}

#[tauri::command]
pub async fn read_file_chunk(
    path: String,
    offset: u64,
    size: usize,
) -> Result<Option<String>, String> {
    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    file.seek(SeekFrom::Start(offset))
        .map_err(|e| e.to_string())?;

    let mut buffer = vec![0u8; size];
    let bytes_read = file.read(&mut buffer).map_err(|e| e.to_string())?;
    if bytes_read == 0 {
        return Ok(None);
    }
    buffer.truncate(bytes_read);
    let content = String::from_utf8_lossy(&buffer).into_owned();
    Ok(Some(content))
}

#[tauri::command]
pub async fn get_file_size(path: String) -> Result<u64, String> {
    let meta = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    Ok(meta.len())
}
