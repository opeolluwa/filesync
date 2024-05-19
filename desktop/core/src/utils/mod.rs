use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{self};
use std::fs::File;
use std::path::{Path, PathBuf};
use ts_rs::TS;

pub mod fs;
pub mod shell;
pub mod system_info;
/// data structure of response to return from Tauri Core
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommandData<T> {
    pub data: Option<T>,
    pub message: String,
    pub status: bool,
}

pub type ApiResponse<D, E> = Result<CommandData<D>, CommandData<E>>;

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
    /// if the response is ok
    /// returns a CommandData struct
    /// with the data, message, and status
    pub fn ok(message: &str, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// if the response is an error
    /// returns a CommandData struct
    /// with the data, message, and status
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
    let is_dir = std::fs::metadata(file)
        .map_err(|err| format!("Error: Cannot query metadata on {:?}: {}", file, err))?
        .is_dir();
    if is_dir {
        return Err(format!("Error: {:?} is a directory.", file));
    }
    Ok(())
}

pub fn generate_password() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "23456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
        .chars()
        .collect();
    const PASSWORD_LENGTH: usize = 8;
    let mut password: Vec<char> = vec!['\0'; PASSWORD_LENGTH];
    for i in 0..PASSWORD_LENGTH {
        let current_char_index = rng.gen_range(0..chars.len());
        password[i] = chars[current_char_index];
    }
    String::from_iter(password)
}

pub fn hash_file(filename: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(filename)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize().to_vec())
}

pub fn expand_dir(dir: PathBuf) -> (Vec<String>, Vec<PathBuf>) {
    let mut files_found = vec![];
    let mut dirs_to_search = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    dirs_to_search.push(entry.path());
                }
                if metadata.is_file() {
                    files_found.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    (files_found, dirs_to_search)
}
