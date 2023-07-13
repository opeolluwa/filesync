// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;

use lazy_static::lazy_static;

use crate::command::{
    audio::fetch_audio_files,
    connect_with_qr_code::generate_qr_code,
    documents::fetch_documents,
    hotspot::{create_wifi_hotspot, kill_wifi_hotspot},
    image::fetch_images,
    search::search_home_dir,
    send_file::share_file_with_peer,
    utils::{close_splashscreen, get_ip_address, get_system_information},
    video::fetch_video_files,
};

mod command;
mod net;
mod server;
mod utils;

// allow sharing of the port
lazy_static! {
    pub static ref SERVER_PORT: u16 =
        portpicker::pick_unused_port().expect("failed to get an unused port");
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("wi-share");
}

fn main() -> Result<(), tauri::Error> {
    // let sys_info = get_system_information();
    // println!(" sys info{:#?}", sys_info);
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(server::core_server());
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_sqlite::init())
        .invoke_handler(tauri::generate_handler![
            get_ip_address,
            fetch_audio_files,
            fetch_video_files,
            fetch_images,
            fetch_video_files,
            close_splashscreen,
            share_file_with_peer,
            get_system_information,
            fetch_documents,
            search_home_dir,
            create_wifi_hotspot,
            kill_wifi_hotspot,
            generate_qr_code
        ])
        .run(tauri::generate_context!())
}
