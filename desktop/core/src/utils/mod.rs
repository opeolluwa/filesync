use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::{self};
use std::fs::File;
use std::path::{Path, PathBuf};
use ts_rs::TS;

// pub mod fs;
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
/// Returns error if not
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


/// generate password fro wifi hotspot
pub fn _generate_password() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "0123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
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




