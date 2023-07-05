// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;
use lazy_static::lazy_static;

// use crate::commands::utils::get_system_information;

mod commands;
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
    /*    // create  ap
       let config = net::linux_hotspot::create_ap();
       println!("{:#?}", config);
    */
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(server::core_server());
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_sqlite::init())
        .invoke_handler(tauri::generate_handler![
            commands::utils::get_ip_address,
            commands::audio::fetch_audio_files,
            commands::video::fetch_video_files,
            commands::image::fetch_images,
            commands::video::fetch_video_files,
            commands::utils::close_splashscreen,
            commands::send_file::share_file_with_peer,
            commands::utils::get_system_information,
            commands::documents::fetch_documents,
            commands::search::search_home_dir,
            commands::hotspot::create_ap,
            commands::hotspot::kill_ap,
            commands::connect_with_qr_code::generate_qr_code
        ])
        .run(tauri::generate_context!())
    // .expect("error while running tauri application");
}
