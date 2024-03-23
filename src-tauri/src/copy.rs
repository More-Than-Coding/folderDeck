use fs_extra::dir::{self, CopyOptions};
use serde_json::json;
use std::path::Path;
use tauri::command;

// The original copy_directory function remains unchanged
fn copy_dir(source: &Path, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    dir::copy(source, destination, &options)?;
    Ok(())
}

// This new function is exposed to Tauri as a command
#[command]
pub fn copy_directory(source: String, destination: String) -> Result<serde_json::Value, String> {
    let source_path = Path::new(&source);
    let destination_path = Path::new(&destination);

    // Check if the destination already exists
    if destination_path.exists() {
        return Ok(json!({
            "success": false,
            "exists": true
        }));
    }

    match copy_dir(&source_path, &destination_path) {
        Ok(_) => Ok(json!({
            "success": true,
            "exists": false
        })),
        Err(e) => {
            // Attempt to downcast the error to an io::Error
            if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                if io_error.kind() == std::io::ErrorKind::AlreadyExists {
                    return Ok(json!({
                        "success": false,
                        "exists": true
                    }));
                }
            }
            Err(e.to_string())
        }
    }
}
