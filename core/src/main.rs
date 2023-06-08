// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;
use lazy_static::lazy_static;

use crate::commands::image::fetch_images;
use crate::commands::utils::get_system_information;

// tauri APIs
use crate::commands::{
    audio::fetch_audio_files,
    documents::fetch_documents,
    send_file::share_file_with_peer,
    utils::{close_splashscreen, get_ip_address},
    video::fetch_video_files,
};

mod commands;
mod server;
mod utils;

// allow sharing of the port
lazy_static! {
    pub static ref SERVER_PORT: u16 =
        portpicker::pick_unused_port().expect("failed to get an unused port");
}

fn main() {
    let img = fetch_video_files().ok();

    println!("images here{:?}", img);
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(server::core_server());
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            get_ip_address,
            fetch_audio_files,
            fetch_video_files,
            fetch_images,
            fetch_video_files,
            close_splashscreen,
            share_file_with_peer,
            get_system_information,
            fetch_documents
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
