use glob::glob;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
extern crate dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandData<T> {
    data: Option<T>,
    message: String,
    status: bool,
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
    fn new(message: &str, status: bool, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            status,
        }
    }
}
// get the ip address of the machine
#[tauri::command]
pub fn get_ip_addr() -> String {
    local_ip().unwrap().to_string()
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// get the audio file form the
#[tauri::command]
pub fn fetch_audio_files() -> Result<CommandData<Vec<PathBuf>>, CommandData<()>> {
    let audio_dir = dirs::audio_dir();

    // if there is an error getting the audio path, fire an error
    let Some(audio_dir) = audio_dir else{
        return Err(CommandData::new("error getting the audio dir", false, ()));
    };

    let mut entries: Vec<PathBuf> = vec![];
    for entry in glob(&format!("{:?}/*", audio_dir)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => entries.push(path),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(CommandData::new("retrieved all audio files", true, entries))
}

// get the video files
// #[tauri::command]
// async fn fetch_video_files()  {

// }

// get
