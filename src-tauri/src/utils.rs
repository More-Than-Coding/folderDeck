use std::sync::Arc;
use crate::structs::{FileInfo,PaginatedResponse};

pub fn paginate<T: Clone>(items: &Arc<Vec<T>>, page: usize, page_size: usize) -> PaginatedResponse<T> {
    let total_items = items.len();
    let pages_total = (total_items + page_size - 1) / page_size;
    let page_current = page.min(pages_total);

    let start = page * page_size;
    let end = if start > total_items {
        total_items
    } else {
        (start + page_size).min(total_items)
    };

    PaginatedResponse {
        items: if start < total_items { items[start..end].to_vec() } else { Vec::new() },
        pages_total,
        page_current,
    }
}


pub fn recursive_files(file_info: &FileInfo, files: &mut Vec<FileInfo>) {
    if file_info.metadata.is_dir {
        if let Some(children) = &file_info.children {
            for child in children {
                recursive_files(child, files);
            }
        }
    } else {
        files.push(file_info.clone()); // Clone the FileInfo object
    }
}

pub fn recursive_search(file_info: &FileInfo, search_string: &str, results: &mut Vec<FileInfo>, limit: usize) {
  // Stop if the limit is reached
  if results.len() >= limit {
      return;
  }

  // Check the current file or directory name
  if file_info.name.to_lowercase().contains(&search_string.to_lowercase()) {
      results.push(file_info.clone()); // Clone to avoid lifetime issues
  }

  // Recursively search the first level of children only
  if let Some(children) = &file_info.children {
      for child in children {
          recursive_search(child, search_string, results, limit);

          // Stop if the limit is reached
          if results.len() >= limit {
              return;
          }
      }
  }
}

use chrono::prelude::*;
use std::time::SystemTime;

pub fn timestamp_log(message: &str)  {
    let now = SystemTime::now();
    let date_time: DateTime<Utc> = now.into();
    println!("{message} - {}", date_time.format("%Y-%m-%d %H:%M:%S%.3f"));
}