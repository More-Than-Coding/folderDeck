use tauri::command;

#[command]
pub fn open_directory(path: &str) {
    use std::process::Command;
    Command::new("open")
        .arg(path)
        .spawn()
        .expect("Failed to open directory");
}