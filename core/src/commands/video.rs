use filesize::PathExt;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::utils::{compute_file_size, is_hidden, CommandData};

// the audio file interface
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoFile {
    file_name: String,
    file_format: String,
    file_path: PathBuf,
    file_size: String,
}

// implement display for the VideoFiles
impl fmt::Display for VideoFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {},
            format: {},
            path: {:?},
            size: {}",
            self.file_name, self.file_format, self.file_path, self.file_size
        )
    }
}

// get the video file form the default video dir of the OS
// return an instance of the CommandData and vector of the path if any
#[tauri::command]
pub fn fetch_video_files() -> Result<CommandData<Vec<VideoFile>>, CommandData<()>> {
    // if there is an error getting the audio path, fire an error
    let video_dir = dirs::video_dir();
    let Some(video_dir) = video_dir else{
        return Err(CommandData::err("error getting the video dir",  ()));
    };

    let mut entries: Vec<VideoFile> = vec![];
    let video_dir = WalkDir::new(video_dir).into_iter();

    for entry in video_dir.filter_entry(|entry| !is_hidden(entry)) {
        let file = entry.unwrap();

        let file_path = &file.path().display().to_string();
        let file_name = file.file_name().to_str().unwrap();
        let file_size: u128 = Path::new(file.path()).size_on_disk().unwrap_or(0).into();
        let file_format = Path::new(file.path())
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let audio_file = VideoFile {
            file_path: file_path.into(),
            file_name: file_name.into(),
            file_format: file_format.to_string(),
            file_size: compute_file_size(file_size),
            ..Default::default()
        };
        entries.push(audio_file);
    }

    Ok(CommandData::ok("retrieved all video files", entries))
}

#[cfg(test)]
mod tests {
    use crate::commands::video::fetch_video_files;
    #[test] // see if there are files in the audio directory path
    fn _fetch_video_files_() {
        let aud_files = fetch_video_files().ok();
        assert!(aud_files.is_some())
    }
}
