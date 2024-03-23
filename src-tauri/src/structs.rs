use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct CombinedResponse {
    pub files_recent: PaginatedResponse<FileInfo>,
    pub projects_name: PaginatedResponse<FileInfo>,
    pub projects_recent: PaginatedResponse<FileInfo>,
}

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

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page_current: usize,
    pub pages_total: usize,
}