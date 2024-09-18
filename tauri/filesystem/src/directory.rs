//! Manages files and directory
//! 
//! 


extern crate dirs;

use filesize::PathExt;
use path_absolutize::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use ts_rs::TS;


/// the dir enum, reads, $HOME, $PICTURES, $VIDEOS, $DOCUMENTS, $DOWNLOADS, and // Other
/// the other is a unit struct that contains a path of the directory to be red
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub(crate) enum Dir {
    /// the device home directory
    Home,
    Pictures,
    Videos,
    Documents,
    Downloads,
    Audio,
    Desktop,
    /// the location the files received are saved
    FileSync,
    Other(String),
}

impl Dir {
    /// to string
    pub fn _to_string(&self) -> String {
        match self {
            Dir::Home => dirs::home_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Pictures => dirs::picture_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Videos => dirs::video_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Documents => dirs::document_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Downloads => dirs::download_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Audio => dirs::audio_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Desktop => dirs::desktop_dir().unwrap().to_str().unwrap().to_string(),
            Dir::FileSync => "filesync".to_string(),
            Dir::Other(path) => path.to_string(),
        }
    }

    // convert to path
    pub fn to_path(&self) -> std::path::PathBuf {
        match self {
            Dir::Home => dirs::home_dir().unwrap(),
            Dir::Pictures => dirs::picture_dir().unwrap(),
            Dir::Videos => dirs::video_dir().unwrap(),
            Dir::Documents => dirs::document_dir().unwrap(),
            Dir::Downloads => dirs::download_dir().unwrap(),
            Dir::Audio => dirs::audio_dir().unwrap(),
            Dir::Desktop => dirs::desktop_dir().unwrap(),
            Dir::FileSync => PathBuf::from("filesync".to_string()),//TODO USE A CONSTANT
            Dir::Other(path) => PathBuf::from(path),
        }
    }

    // from string
    pub fn from_string(path: &str) -> Self {
        match path {
            "home" => Dir::Home,
            "pictures" => Dir::Pictures,
            "videos" => Dir::Videos,
            "documents" => Dir::Documents,
            "downloads" => Dir::Downloads,
            "audio" => Dir::Audio,
            "desktop" => Dir::Desktop,
            "filesync" => Dir::FileSync,
            _ => Dir::Other(path.to_string()),
        }
    }
}

// the file structure
#[derive(Debug, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct File {
    pub file_name: String,
    pub file_format: String,
    file_path: PathBuf,
    file_size: String,
    pub is_hidden: bool,
    pub is_folder: bool,
}

impl File {
    // from path
    pub fn from_path(path: &PathBuf) -> Self {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_path = path.display().to_string();
        let mut file_size: u128 = path.size_on_disk().unwrap_or(0).into();
        let file_format = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let is_folder = path.is_dir();

        if is_folder {
            file_size = fs_extra::dir::get_size(path).unwrap_or(0) as u128;
        }
        let is_hidden = path
            .file_name()
            .unwrap()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap();

        Self {
            file_name: file_name.into(),
            file_path: file_path.into(),
            file_size: compute_file_size(file_size),
            file_format: file_format.into(),
            is_folder,
            is_hidden,
        }
    }
}

/// a function to compute file size
/// accept files size in byte and parse it to human readable KB, MB, TB, GB e.t.
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

/// get all the files in a directory
/// returns a vector of the file path
pub async fn get_files_in_directory(dir: &Path) -> Result<Vec<String>, String> {
    let paths = fs::read_dir(dir).map_err(|err| err.to_string())?;
    let mut files = Vec::new();
    for path in paths {
        let dir_entry_path = path.unwrap().path();
        let absolutized_path = dir_entry_path.absolutize().unwrap();
        let absolute_path = absolutized_path.to_str().unwrap();
        let file_path = absolute_path.to_string();
        files.push(file_path);
    }

    Ok(files)
}
