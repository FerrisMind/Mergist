#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod converter;
mod github;
mod models;

use commands::{
    ConversionState, cancel_conversion, convert_repo_to_markdown, export_issues, get_file_size,
    read_file_chunk,
};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .manage(ConversionState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            convert_repo_to_markdown,
            export_issues,
            cancel_conversion,
            read_file_chunk,
            get_file_size
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            if let Some(window) = app.get_webview_window("main") {
                // Авто-открытие DevTools только в dev-сборках
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
