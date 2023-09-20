// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;

use lazy_static::lazy_static;
use server::http_server;

use crate::api::{
    fs::{
        audio::fetch_audio, document::fetch_documents, get_transfer_history, image::fetch_images,
        persist_transfer_history, search_home_dir, share_file_with_peer, video::fetch_videos,
    },
    settings::{get_settings, update_settings},
    utils::{generate_qr_code, get_ip_address, get_system_information},
    wifi::{create_wifi_hotspot, kill_wifi_hotspot},
};

mod api;
mod app_state;
mod database;
mod fs;
mod server;
mod utils;
mod wifi;

// allow sharing of the port
lazy_static! {
    pub static ref SERVER_PORT: u16 =
        portpicker::pick_unused_port().expect("failed to get an unused port");
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("filesync");

    /* create a database in the home dir and / save files to $HOME/filesync/.dat */
     pub static ref DB_URL: std::string::String = {
        let os_default_downloads_dir = dirs::download_dir().unwrap();
        let db_path = format!(
            "{downloads_dir}/{db_path}",
            downloads_dir = os_default_downloads_dir.display(),
            db_path = "store"
        );
        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/filesync.db")
    };
}

fn main() -> Result<(), tauri::Error> {
    let state = app_state::State {
        ..Default::default()
    };
  
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(http_server::core_server());

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            fetch_audio,
            fetch_videos,
            fetch_images,
            fetch_documents,
            create_wifi_hotspot,
            kill_wifi_hotspot,
            generate_qr_code,
            get_ip_address,
            get_system_information,
            search_home_dir,
            persist_transfer_history,
            share_file_with_peer,
            get_settings,
            update_settings,
            get_transfer_history,
        ])
        .run(tauri::generate_context!())
}
