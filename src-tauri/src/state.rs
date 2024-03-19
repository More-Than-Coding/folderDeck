use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::command;
use tokio::sync::Mutex as AsyncMutex;
use crate::read::FileInfo;

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page_current: usize,
    pub pages_total: usize,
}

// Global Variables
lazy_static! {
    pub static ref FILE_DATA: Mutex<Arc<HashMap<String, FileInfo>>> = Mutex::new(Arc::new(HashMap::new()));
    pub static ref SORTED_DIR_CACHE: AsyncMutex<Arc<Vec<FileInfo>>> = AsyncMutex::new(Arc::new(Vec::new()));
    pub static ref SORTED_DIR_CACHE_BY_MOD: AsyncMutex<Arc<Vec<FileInfo>>> = AsyncMutex::new(Arc::new(Vec::new()));
    pub static ref SORTED_FILES_CACHE_BY_MOD: AsyncMutex<Arc<Vec<FileInfo>>> = AsyncMutex::new(Arc::new(Vec::new()));
}

// Methods
pub fn update_file_data(path: &Path, data: FileInfo) {
    let mut file_data = FILE_DATA.lock().unwrap();
    let path_str = path.to_string_lossy().to_string();
    Arc::get_mut(&mut *file_data).unwrap().insert(path_str, data);

    // Trigger async update of caches
    tokio::spawn(async {
        update_sorted_dir_cache().await;
        update_sorted_dir_cache_by_mod().await;
        update_sorted_files_cache_by_mod().await;
    });
}

// Supporting Methods
fn collect_files_recursive(file_info: &FileInfo, files: &mut Vec<FileInfo>) {
    if file_info.metadata.is_dir {
        if let Some(children) = &file_info.children {
            for child in children {
                collect_files_recursive(child, files);
            }
        }
    } else {
        files.push(file_info.clone());
    }
}

fn paginate<T: Clone>(items: &Arc<Vec<T>>, page: usize, page_size: usize) -> PaginatedResponse<T> {
    let total_items = items.len();
    let pages_total = (total_items + page_size - 1) / page_size;  // Calculate total number of pages
    let page_current = page.min(pages_total);  // Ensure page_current is not out of bounds

    let start = page * page_size;
    let end = start + page_size.min(total_items - start);

    PaginatedResponse {
        items: if start < total_items { items[start..end].to_vec() } else { Vec::new() },
        pages_total,
        page_current,
    }
}

fn search_files_recursive(file_info: &FileInfo, search_string: &str, results: &mut Vec<FileInfo>, limit: usize) {
    if results.len() >= limit {
        return; // Stop if the limit is reached
    }

    // Check the current file or directory name
    if file_info.name.to_lowercase().contains(&search_string.to_lowercase()) {
        results.push(file_info.clone()); // Clone to avoid lifetime issues
    }

    // Recursively search the first level of children only
    if let Some(children) = &file_info.children {
        for child in children {
            search_files_recursive(child, search_string, results, limit);
            if results.len() >= limit {
                return; // Stop if the limit is reached
            }
        }
    }
}


// Creating Caches
async fn update_sorted_dir_cache() {
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

async fn update_sorted_dir_cache_by_mod() {
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

async fn update_sorted_files_cache_by_mod() {
    let mut all_files: Vec<FileInfo> = Vec::new();

    {
        let file_data_lock = FILE_DATA.lock().unwrap();
        for file_info in file_data_lock.values() {
            collect_files_recursive(file_info, &mut all_files);
        }
    }  // Lock is dropped here

    // Sort files by their modified timestamp in descending order (most recent first)
    all_files.sort_by(|a, b| b.metadata.modified.cmp(&a.metadata.modified));

    let mut cache = SORTED_FILES_CACHE_BY_MOD.lock().await;
    *cache = Arc::new(all_files);
}


// Tauri Commands
#[command]
pub async fn files_recent(page: usize, page_size: usize) -> Result<PaginatedResponse<FileInfo>, String> {
    let cache = SORTED_FILES_CACHE_BY_MOD.lock().await;

    // Execute pagination logic in an immediately awaited async block
    Ok(async {
        paginate(&cache, page, page_size)
    }.await)
}

#[command]
pub async fn projects_name(page: usize, page_size: usize) -> Result<PaginatedResponse<FileInfo>, String> {
    let cache = SORTED_DIR_CACHE.lock().await;

    // Execute pagination logic in an immediately awaited async block
    Ok(async {
        paginate(&cache, page, page_size)
    }.await)
}

#[command]
pub async fn projects_recent(page: usize, page_size: usize) -> Result<PaginatedResponse<FileInfo>, String> {
    let cache = SORTED_DIR_CACHE_BY_MOD.lock().await;

    // Execute pagination logic in an immediately awaited async block
    Ok(async {
        paginate(&cache, page, page_size)
    }.await)
}

#[command]
pub async fn search(query: String, limit: usize) -> Result<Vec<FileInfo>, String> {
    let mut matches = Vec::new();

    {
        let file_data_lock = FILE_DATA.lock().unwrap();
        let file_data_values: Vec<FileInfo> = file_data_lock.values().cloned().collect(); // Clone to detach from the lock's lifetime

        // Perform the search on the first level children of each top-level FileInfo
        for file_info in file_data_values.iter() {
            search_files_recursive(file_info, &query, &mut matches, limit);
            if matches.len() >= limit {
                break; // Stop if the limit is reached
            }
        }
    } // Lock is dropped here

    Ok(matches)
}

#[command]
pub async fn reset_caches() {
    // First, deal with the synchronous std::sync::Mutex
    // Ensure the lock is dropped immediately after use by scoping the block
    {
        let mut file_data_lock = FILE_DATA.lock().unwrap(); // Consider handling the potential panic more gracefully
        *file_data_lock = Arc::new(HashMap::new());
    } // The lock is dropped here

    // Now, proceed with the async locks, which can safely be used with await
    // Reset SORTED_DIR_CACHE
    let mut sorted_dir_cache_lock = SORTED_DIR_CACHE.lock().await;
    *sorted_dir_cache_lock = Arc::new(Vec::new());

    // Reset SORTED_DIR_CACHE_BY_MOD
    let mut sorted_dir_cache_by_mod_lock = SORTED_DIR_CACHE_BY_MOD.lock().await;
    *sorted_dir_cache_by_mod_lock = Arc::new(Vec::new());

    // Reset SORTED_FILES_CACHE_BY_MOD
    let mut sorted_files_cache_by_mod_lock = SORTED_FILES_CACHE_BY_MOD.lock().await;
    *sorted_files_cache_by_mod_lock = Arc::new(Vec::new());
}
