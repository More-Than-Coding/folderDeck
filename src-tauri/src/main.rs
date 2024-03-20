// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::LogTarget;
use log::LevelFilter;
use std::env;

pub mod state;
pub mod copy;
pub mod open;
pub mod read;

// Methods
fn get_aptabase_id() -> String {
    env::var("APTABASE_ID").unwrap_or_else(|_| "A-US-1234567890".to_string())
}
// Main Launch
#[tokio::main]
async fn main() {
    let aptabase_id = get_aptabase_id();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            copy::copy_directory,
            open::open_directory,
            read::read_directory,
            state::files_recent,
            state::projects_name,
            state::projects_recent,
            state::reset_caches,
            state::search,
        ])
        .plugin(tauri_plugin_aptabase::Builder::new(&aptabase_id).build())
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout])
                .level(LevelFilter::Info)
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
