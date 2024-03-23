use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::sync::Arc;
use tauri::command;

use crate::data::{update_file_data, FILE_DATA, SORTED_DIR_CACHE, SORTED_DIR_CACHE_BY_MOD, SORTED_FILES_CACHE_BY_MOD};
use crate::structs::{FileInfo, FileMetadata};

// Methods
fn update_projects_dir(path: &Path, ignore: &[String]) -> Result<FileInfo, std::io::Error> {
    let metadata = fs::symlink_metadata(path)?;  // Use symlink_metadata to detect symlinks
    let is_file = metadata.is_file();
    let mut children = None;

    // Convert SystemTime to milliseconds since UNIX epoch
    let created = metadata.created().ok().and_then(|time| time.duration_since(UNIX_EPOCH).ok()).map(|duration| duration.as_millis() as u64);
    let modified = metadata.modified().ok().and_then(|time| time.duration_since(UNIX_EPOCH).ok()).map(|duration| duration.as_millis() as u64);

    // Calculate file size for files, None for directories
    let file_size = if is_file { Some(metadata.len()) } else { None };

    if path.is_dir() && !metadata.file_type().is_symlink() {  // Check if the path is a directory and not a symlink
        let entries = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let file_path = entry.path().to_string_lossy().to_lowercase();
                !ignore.iter().any(|i| file_path.contains(&i.to_lowercase()))
            })
            .map(|entry| {
                let entry_metadata = fs::symlink_metadata(&entry.path())?;  // Check each entry for symlink
                if entry_metadata.file_type().is_symlink() {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Skipping symlink"))
                } else {
                    update_projects_dir(&entry.path(), ignore)
                }
            })
            .filter_map(Result::ok)  // Optionally, you can handle errors differently here
            .collect::<Vec<_>>();

        if !entries.is_empty() {
            children = Some(entries);
        }
    }

    Ok(FileInfo {
        name: path.file_name().unwrap_or_else(|| path.as_os_str()).to_string_lossy().into_owned(),
        path: path.to_string_lossy().into_owned(),
        metadata: FileMetadata {
            created,
            modified,
            is_file,
            is_dir: path.is_dir() && !metadata.file_type().is_symlink(),  // Ensure it's not a symlink
            file_size,
        },
        children,
    })
}


// Tauri Commands
#[command]
pub fn update_projects(path: String, ignore: Vec<String>) -> Result<HashMap<String, usize>, String> {
    let path = Path::new(&path);
    match update_projects_dir(path, &ignore) {
        Ok(data) => {
            update_file_data(path, data.clone());

            // Initialize the children count to 0
            let children_count = data.children.as_ref().map_or(0, |children| children.len());

            // Return the children count with the key "project"
            Ok(HashMap::from([("projects".to_string(), children_count)]))
        },
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn reset_projects() {
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