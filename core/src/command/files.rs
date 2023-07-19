use crate::{files, utils::CommandData};

/// get the list of the audio files
#[tauri::command]
pub async fn get_audio_files() -> Result<CommandData<Vec<files::File>>, CommandData<()>> {
    files::audio::get_audio_files()
}

/// get the list of the documents
#[tauri::command]
pub async fn get_documents() -> Result<CommandData<Vec<files::File>>, CommandData<()>> {
    files::documents::get_documents()
}

/// get the list of the images
#[tauri::command]
pub async fn get_images() -> Result<CommandData<Vec<files::File>>, CommandData<()>> {
    files::images::get_images()
}

/// get the list of the videos
#[tauri::command]
pub async fn get_videos() -> Result<CommandData<Vec<files::File>>, CommandData<()>> {
    files::video::get_videos()
}

