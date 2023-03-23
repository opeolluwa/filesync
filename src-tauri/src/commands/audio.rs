use filesize::PathExt;
use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use super::CommandData;

// the audio file interface
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFile {
    file_name: String,
    file_format: String,
    file_path: PathBuf,
    file_size: u128,
}
// AudioFile constructor
impl AudioFile {
    pub fn new(name: &str, format: &str, path: PathBuf, size: u128) -> Self {
        Self {
            file_name: name.to_string(),
            file_format: format.to_string(),
            file_path: path,
            file_size: size,
        }
    }
}
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
    let audio_dir = dirs::audio_dir();

    // if there is an error getting the audio path, fire an error
    let Some(audio_dir) = audio_dir else{
        return Err(CommandData::new("error getting the audio dir", false, ()));
    };

    //
    let mut entries: Vec<AudioFile> = vec![];
    for entry in fs::read_dir(audio_dir).expect("error reading file") {
        let dir = entry.expect("could not read dir");
        let file = &dir.path();

        let file_name = Path::new(file)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let file_extension = Path::new(file)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let file_size: u128 = file.size_on_disk().unwrap_or(0).into();
        let file_path = &dir.path();

        let audio_file = AudioFile::new(
            file_name,
            file_extension,
            file_path.to_path_buf(),
            file_size,
        );
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
