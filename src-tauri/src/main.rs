// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    // let aud_files = commands::fetch_audio_files().ok().unwrap();
    // println!("the audio files {:?}", aud_files.data.unwrap()[6]);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_ip_addr,
            commands::fetch_audio_files,
            commands::fetch_video_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
