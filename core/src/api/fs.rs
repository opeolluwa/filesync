use std::path::PathBuf;

use crate::{
    fs::{file::File, file_manager},
    utils::{ApiResponse, CommandData},
};
use dirs;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// the dir enum, reads, $HOME, $PICTURES, $VIDEOS, $DOCUMENTS, $DOWNLOADS, and // Other
/// the other is a unit struct that contains a path of the directory to be red
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub(crate) enum Dir {
    Home,
    Pictures,
    Videos,
    Documents,
    Downloads,
    Audio,
    Desktop,
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
            Dir::Other(path) => std::path::PathBuf::from(path),
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
            "desktop"=>Dir::Desktop,
            _ => Dir::Other(path.to_string()),
        }
    }
}
/// read directory
#[tauri::command]
pub async fn read_dir(path: &str) -> ApiResponse<Vec<File>, ()> {
    let path = Dir::from_string(path).to_path();
    let files = file_manager::get_files_in_directory(&path).await;
    if files.is_err() {
        return Err(CommandData::err("Error fetching files", ()));
    }

    // convert to file type
    let mut entries: Vec<File> = Vec::new();
    for entry in files.unwrap() {
        let file_path =  std::path::PathBuf::from(entry);
        let file = File::from_path(&file_path);
        entries.push(file)
    }

    Ok(CommandData::ok("Successfully fetch the data", entries))
}
