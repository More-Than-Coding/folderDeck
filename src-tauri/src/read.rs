use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use tauri::command;
use crate::state::update_file_data;

// Data Types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub metadata: FileMetadata,
    pub children: Option<Vec<FileInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub created: Option<u64>,
    pub modified: Option<u64>,
    pub is_file: bool,
    pub is_dir: bool,
    pub file_size: Option<u64>,
}

// Methods
fn read_dir(path: &Path, ignore: &[String]) -> Result<FileInfo, std::io::Error> {
  let metadata = fs::metadata(path)?;
  let is_file = metadata.is_file();
  let mut children = None;

  // Convert SystemTime to milliseconds since UNIX epoch
  let created = metadata.created().ok().and_then(|time| time.duration_since(UNIX_EPOCH).ok()).map(|duration| duration.as_millis() as u64);
  let modified = metadata.modified().ok().and_then(|time| time.duration_since(UNIX_EPOCH).ok()).map(|duration| duration.as_millis() as u64);

  // Calculate file size for files, None for directories
  let file_size = if is_file { Some(metadata.len()) } else { None };

  if path.is_dir() {
      let entries = fs::read_dir(path)?
          .filter_map(|entry| entry.ok())
          .filter(|entry| {
            let file_path = entry.path().to_string_lossy().to_lowercase();
            !ignore.iter().any(|i| file_path.contains(&i.to_lowercase()))
        })
          .map(|entry| read_dir(&entry.path(), ignore))
          .collect::<Result<Vec<_>, std::io::Error>>()?;

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
          is_dir: path.is_dir(),
          file_size,  // Set file_size here
      },
      children,
  })
}

// Tauri Commands
#[command]
pub fn read_directory(path: String, ignore: Vec<String>) -> Result<HashMap<String, usize>, String> {
    let path = Path::new(&path);
    match read_dir(path, &ignore) {
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
