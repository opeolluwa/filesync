use std::{path::PathBuf, fs};

use super::CommandData;

// get the video files
#[tauri::command]
pub fn fetch_video_files() -> Result<CommandData<Vec<PathBuf>>, CommandData<()>> {
    let video_dir = dirs::video_dir();

    // if there is an error getting the video path, fire an error
    let Some(video_dir) = video_dir else{
        return Err(CommandData::new("error reading the video dir", false, ()));
    };

    //
    let mut entries: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(video_dir).expect("error reading file") {
        let dir = entry.expect("could not read dir");
        entries.push(dir.path());
    }
    Ok(CommandData::new("retrieved all audio files", true, entries))
}


// #[cfg(test)]
// mod tests {
//     use crate::commands::{self, audio::AudioFile, video::fetch_video_files};

//     #[test] // see if there are files in the video directory path
//     fn _fetch_video_files_() {
//         let vid_files: Option<commands::CommandData<Vec<AudioFile>>> = fetch_video_files().ok();
//         assert!(vid_files.is_some())
//     }
// }
