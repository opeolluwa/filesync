// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;
use lazy_static::lazy_static;

// use crate::commands::utils::get_system_information;

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
    // create  ap
    /*    let config = net::linux_hotspot::create_hotspot();
       println!("{:#?}", config);
    */
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(server::core_server());
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_sqlite::init())
        .invoke_handler(tauri::generate_handler![
            command::utils::get_ip_address,
            command::audio::fetch_audio_files,
            command::video::fetch_video_files,
            command::image::fetch_images,
            command::video::fetch_video_files,
            command::utils::close_splashscreen,
            command::send_file::share_file_with_peer,
            command::utils::get_system_information,
            command::documents::fetch_documents,
            command::search::search_home_dir,
            command::hotspot::create_wifi_hotspot,
            command::hotspot::kill_wifi_hotspot,
            command::connect_with_qr_code::generate_qr_code
        ])
        .run(tauri::generate_context!())
    // .expect("error while running tauri application");
}
