// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::LogTarget;
use log::LevelFilter;
use std::env;

pub mod copy;
pub mod data;
pub mod open;
pub mod runtime;
pub mod structs;
pub mod update;
pub mod utils;

// Methods
fn get_aptabase_id() -> String {
    // Attempt to get the compile-time value of APTABASE_ID
    let compile_time_value = option_env!("APTABASE_ID");

    match compile_time_value {
        // If available at compile time, use it directly
        Some(value) => value.to_string(),
        // If not available at compile time, attempt to get it at runtime, with a fallback
        None => env::var("APTABASE_ID").unwrap_or_else(|_| "A-US-1234567890".to_string()),
    }
}

// Main Launch
#[tokio::main]
async fn main() {
    let aptabase_id = get_aptabase_id();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            copy::copy_directory,
            open::open_directory,
            data::fetch_all_data,
            data::files_recent,
            data::projects_name,
            data::projects_recent,
            data::search,
            update::reset_projects,
            update::update_projects,
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
