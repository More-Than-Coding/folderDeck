use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::async_runtime;
use tauri::command;
use tokio::sync::Mutex as AsyncMutex;

use crate::structs::{CombinedResponse,FileInfo,PaginatedResponse};
use crate::utils::{paginate,recursive_files,recursive_search};


// Cached Data
lazy_static! {
    pub static ref FILE_DATA: Mutex<Arc<HashMap<String, FileInfo>>> = Mutex::new(Arc::new(HashMap::new()));
    pub static ref SORTED_DIR_CACHE: AsyncMutex<Arc<Vec<FileInfo>>> = AsyncMutex::new(Arc::new(Vec::new()));
    pub static ref SORTED_DIR_CACHE_BY_MOD: AsyncMutex<Arc<Vec<FileInfo>>> = AsyncMutex::new(Arc::new(Vec::new()));
    pub static ref SORTED_FILES_CACHE_BY_MOD: AsyncMutex<Arc<Vec<FileInfo>>> = AsyncMutex::new(Arc::new(Vec::new()));
}

// Update Cached Data
async fn update_dir_name() {
    let all_children: Vec<FileInfo> = {
        let file_data_lock = FILE_DATA.lock().unwrap();
        file_data_lock.values()
            .filter(|info| info.metadata.is_dir && info.children.is_some())
            .flat_map(|dir| dir.children.as_ref().unwrap().clone())
            .collect()
    };

    let mut sorted_children = all_children;
    sorted_children.sort_by(|a, b| a.name.to_uppercase().cmp(&b.name.to_uppercase()));

    let mut cache = SORTED_DIR_CACHE.lock().await;
    *cache = Arc::new(sorted_children);
}

async fn update_dir_recent() {
    let all_children: Vec<FileInfo> = {
        let file_data_lock = FILE_DATA.lock().unwrap();
        file_data_lock.values()
            .filter(|info| info.metadata.is_dir && info.children.is_some())
            .flat_map(|dir| dir.children.as_ref().unwrap().clone())
            .collect()
    };

    let mut sorted_children = all_children;
    sorted_children.sort_by(|a, b| b.metadata.modified.cmp(&a.metadata.modified));

    let mut cache = SORTED_DIR_CACHE_BY_MOD.lock().await;
    *cache = Arc::new(sorted_children);
}

async fn update_files_recent() {
    let all_files_collected: Vec<FileInfo> = {
        let file_data_lock = FILE_DATA.lock().unwrap();
        file_data_lock.values()
            .flat_map(|file_info| {
                let mut files = Vec::new();
                recursive_files(file_info, &mut files);
                files.into_iter()
            })
            .collect()
    };

    let mut all_files_sorted = all_files_collected;
    all_files_sorted.sort_by(|a, b| b.metadata.modified.cmp(&a.metadata.modified));

    let mut cache = SORTED_FILES_CACHE_BY_MOD.lock().await;
    *cache = Arc::new(all_files_sorted);
}


pub async fn update_file_data(path: &Path, data: FileInfo) {
    let mut file_data = FILE_DATA.lock().unwrap();
    let path_str = path.to_string_lossy().to_string();
    Arc::get_mut(&mut *file_data).unwrap().insert(path_str, data);

    async_runtime::spawn(async {
        update_dir_name().await;
        update_dir_recent().await;
        update_files_recent().await;
    });
}

// Tauri Commands
#[command]
pub async fn files_recent(page: usize, page_size: usize) -> Result<PaginatedResponse<FileInfo>, String> {
    let cache = SORTED_FILES_CACHE_BY_MOD.lock().await;
    Ok(async { paginate(&cache, page, page_size) }.await)
}

#[command]
pub async fn projects_name(page: usize, page_size: usize) -> Result<PaginatedResponse<FileInfo>, String> {
    let cache = SORTED_DIR_CACHE.lock().await;
    Ok(async { paginate(&cache, page, page_size) }.await)
}

#[command]
pub async fn projects_recent(page: usize, page_size: usize) -> Result<PaginatedResponse<FileInfo>, String> {
    let cache = SORTED_DIR_CACHE_BY_MOD.lock().await;
    Ok(async { paginate(&cache, page, page_size) }.await)
}

#[command]
pub async fn fetch_all_data(page: usize, page_size: usize) -> Result<CombinedResponse, String> {
    // You might need to adjust the locking logic depending on your actual cache structure
    let files_recent_future = files_recent(page, page_size);
    let projects_name_future = projects_name(page, page_size);
    let projects_recent_future = projects_recent(page, page_size);

    // Await all futures concurrently
    let (files_recent_result, projects_name_result, projects_recent_result) = tokio::try_join!(
        files_recent_future,
        projects_name_future,
        projects_recent_future
    )?;

    Ok(CombinedResponse {
        files_recent: files_recent_result,
        projects_name: projects_name_result,
        projects_recent: projects_recent_result,
    })
}

#[command]
pub async fn search(query: String, limit: usize) -> Result<Vec<FileInfo>, String> {
    let mut matches = Vec::new();

    {
        let file_data_lock = FILE_DATA.lock().unwrap();
        let file_data_values: Vec<FileInfo> = file_data_lock.values().cloned().collect(); // Clone to detach from the lock's lifetime

        // Perform the search on the first level children of each top-level FileInfo
        for file_info in file_data_values.iter() {
            recursive_search(file_info, &query, &mut matches, limit);
            if matches.len() >= limit {
                break; // Stop if the limit is reached
            }
        }
    }

    Ok(matches)
}

