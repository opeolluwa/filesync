use std::fmt::{self};
use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::DirEntry;

pub mod ip_manager;
pub mod linux_hotspot;
pub mod system_info;
/// a function to compute file size
/// accept files size in byte and parse it to human readable KB, MB, TB, GB e.t.c
pub fn compute_file_size(size: u128) -> String {
    if size > (1024 * 1024 * 1024 * 1024) {
        format!("{:.2} TB", size / (1024 * 1024 * 1024 * 1024))
    } else if size > (1024 * 1024 * 1024) {
        format!("{:.2} GB", size / (1024 * 1024 * 1024))
    } else if size > (1024 * 1024) {
        format!("{:.2} MB", size / (1024 * 1024))
    } else if size > 1024 {
        format!("{:.2} KB", size / (1024))
    } else {
        format!("{:.2} B", size)
    }
}

/// see if file is a dot file eg .cache .yarn
/// ignore if true
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// data structure of response to return from Tauri Core
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandData<T> {
    pub data: Option<T>,
    pub message: String,
    pub status: bool,
}

impl<T: fmt::Display + fmt::Debug> fmt::Display for CommandData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "data: {:?}
            message: {},
            status: {}
            ",
            self.data, self.message, self.status,
        )
    }
}

impl<T> Default for CommandData<T> {
    fn default() -> Self {
        Self {
            data: None::<T>,
            message: String::from("returned data form core"),
            status: true,
        }
    }
}

impl<T> CommandData<T> {
    // if no error
    pub fn ok(message: &str, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            ..Default::default()
        }
    }

    // if error
    pub fn err(message: &str, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            status: false,
        }
    }
}

/// Checks whether given path is a file that can be opened.
///
/// Returns error if no.
pub fn _verify_file_openable(file: &PathBuf) -> Result<(), String> {
    File::open(Path::new(&file))
        .map_err(|err| format!("Error: Cannot open {:?}: {}", file, err))?;
    let is_dir = std::fs::metadata(&file)
        .map_err(|err| format!("Error: Cannot query metadata on {:?}: {}", file, err))?
        .is_dir();
    if is_dir {
        return Err(format!("Error: {:?} is a directory.", file));
    }
    Ok(())
}
