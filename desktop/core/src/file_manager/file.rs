use filesize::PathExt;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, path::Path};
use ts_rs::TS;
use walkdir::DirEntry;
extern crate dirs;
use path_absolutize::*;




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



#[derive(serde::Serialize, Debug)]
pub struct DriveInformation {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
    disk_type: String,
    file_system: String,
}

#[derive(serde::Serialize)]
pub struct Drives {
    array_of_drives: Vec<DriveInformation>,
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

#[allow(unused)]
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
            ..Default::default()
        }
    }

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
