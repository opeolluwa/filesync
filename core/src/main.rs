// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;

use lazy_static::lazy_static;

use crate::{
    command::{
        audio::fetch_audio_files,
        connect_with_qr_code::generate_qr_code,
        documents::fetch_documents,
        // files::{get_audio_files, get_documents, get_images, get_videos},
        hotspot::{create_wifi_hotspot, kill_wifi_hotspot},
        // search::search_downloads_dir,
        send_file::share_file_with_peer,
        utils::{close_splashscreen, get_ip_address, get_system_information},
    },
    server::http_server,
};

mod command;
mod database;
mod files;
mod net;
mod server;
mod utils;
// allow sharing of the port
lazy_static! {
    pub static ref SERVER_PORT: u16 =
        portpicker::pick_unused_port().expect("failed to get an unused port");
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("filesync");

     pub static ref DB_URL: std::string::String = {
        //create "utils" directory in the home dir and / save files to $HOME/utils;
        let os_default_downloads_dir = dirs::download_dir().unwrap();
        let db_path = format!(
            "{downloads_dir}/{upload_dir}",
            downloads_dir = os_default_downloads_dir.display(),
            upload_dir = ".dat"
        );
        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/filesync.db")
    };
}

fn main() -> Result<(), tauri::Error> {
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(http_server::core_server());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_ip_address,
            fetch_documents,
            fetch_audio_files,
            close_splashscreen,
            share_file_with_peer,
            get_system_information,
            fetch_documents,
            create_wifi_hotspot,
            kill_wifi_hotspot,
            generate_qr_code
        ])
        .run(tauri::generate_context!())
}
