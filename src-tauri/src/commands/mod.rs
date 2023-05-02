#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

//exports
pub mod audio;
pub mod documents;
pub mod image;
pub mod send_file;
pub mod utils;
pub mod video;
