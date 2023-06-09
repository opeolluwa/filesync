use filesize::PathExt;
use std::{path::PathBuf};
use walkdir::DirEntry;
use serde::{Deserialize, Serialize};
use crate::utils::compute_file_size;

// the file structure
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub file_name: String,
    pub file_format: String,
    file_path: PathBuf,
    file_size: String,
}

impl File {
    // convert a DirEntry to a File
    pub fn from(entry: DirEntry) -> Self {
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        let file_path = &entry.path().display().to_string();
        let file_size: u128 = entry.path().size_on_disk().unwrap_or(0).into();
        let binding = entry.path();
        let file_format = binding
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        Self {
            file_name: file_name.into(),
            file_path: file_path.into(),
            file_size: compute_file_size(file_size),
            file_format: file_format.into(),
        }
    }
}