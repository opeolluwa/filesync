use filesize::PathExt;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use super::{
    utils::{compute_file_size, is_hidden},
    CommandData,
};

// the audio file interface
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFile {
    file_name: String,
    file_format: String,
    file_path: PathBuf,
    file_size: String,
}
// AudioFile constructor
// impl AudioFile {
//     pub fn _new(name: &str, format: &str, path: PathBuf, size: u128) -> Self {
//         Self {
//             file_name: name.to_string(),
//             file_format: format.to_string(),
//             file_path: path,
//             file_size: size,
//         }
//     }
// }
// implement display for the AudioFiles
impl fmt::Display for AudioFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(name: {}\nformat: {}\npath: {:?}, size\n{})",
            self.file_name, self.file_format, self.file_path, self.file_size
        )
    }
}

// get the audio file form the default audio dir of the OS
// return an instance of the CommandData and vector of the path if any
#[tauri::command]
pub fn fetch_audio_files() -> Result<CommandData<Vec<AudioFile>>, CommandData<()>> {
    // if there is an error getting the audio path, fire an error
    let audio_dir = dirs::audio_dir();
    let Some(audio_dir) = audio_dir else{
        return Err(CommandData::new("error getting the audio dir", false, ()));
    };

    let mut entries: Vec<AudioFile> = vec![];
    let audio_dir = WalkDir::new(audio_dir).into_iter();

    for entry in audio_dir.filter_entry(|entry| !is_hidden(entry)) {
        let file = entry.unwrap();

        let file_path = &file.path().display().to_string();
        let file_name = file.file_name().to_str().unwrap();
        let file_size: u128 = Path::new(file.path()).size_on_disk().unwrap_or(0).into();
        let file_format = Path::new(file.path())
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let audio_file = AudioFile {
            file_path: file_path.into(),
            file_name: file_name.into(),
            file_format: file_format.to_string(),
            file_size: compute_file_size(file_size),
            ..Default::default()
        };
        entries.push(audio_file);
    }

    Ok(CommandData::new("retrieved all audio files", true, entries))
}

#[cfg(test)]
mod tests {
    use crate::commands::audio::fetch_audio_files;
    #[test] // see if there are files in the audio directory path
    fn _fetch_audio_files_() {
        let aud_files = fetch_audio_files().ok();
        assert!(aud_files.is_some())
    }
}
