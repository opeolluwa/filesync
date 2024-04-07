// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;

/**
 * the application is structured thus
 * src
 * __api
 * __server
 * __fs
 * ...
 */


use  crate::api::fs_api::read_dir;
use  crate::api::fs_api::get_transfer_history;
// Import individual items from crate::api::settings
use crate::api::settings::{get_settings, update_settings};

// Import individual items from crate::api::utils
use crate::api::utils::{generate_qr_code, get_ip_address, get_system_information};

// Import individual items from crate::api::wifi
use crate::api::wifi::{create_wifi_hotspot, kill_wifi_hotspot, scan_wifi};

// Import lazy_static crate
use lazy_static::lazy_static;

// Import http_server from server module
use server::http_server;

mod api;
mod app_state;
mod database;
mod fs;
mod server;
mod utils;
mod wifi;

lazy_static! {
    /**
 * the lazy static crate allow the lazy evaluation of constants thus, one can bypass the impossible dynamic bindings of constants
 *
 *
 * Herein the server port made globally available, this allow for ease of sharing same with file upload directory
 */
    pub static ref SERVER_PORT: u16 =
        portpicker::pick_unused_port().expect("failed to get an unused port");
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("filesync");

    /* create a database in the home dir and / save files to $HOME/filesync/.dat */
     pub static ref DB_URL: std::string::String = {
        let os_default_downloads_dir = dirs::download_dir().unwrap();
        let db_path = format!(
            "{downloads_dir}/{db_path}",
            downloads_dir = os_default_downloads_dir.display(),
            db_path = ".dat"
        );
        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/filesync.db")
    };
}

/**
 * the application runs two major thing, a file sever and the tauri runtime responsible for rendering the UI and general UI interactions
 *
 * since both are async operations they have to be run in separate threads
 * this prevent one path blocking the other
 *
 * for a deeper understanding, consider
 * ```rust
 * http::core_server::run().await  // run the http serer
 *
 * tauri::Builder::default().run().await // run the ui process
 *
 * ```
 *
 * That was the prev implementation but it doesn't work, either the server isn't up or the UI is not rendered
 *
 * thus the app was moved to run in separate thread
 */
fn main() -> Result<(), tauri::Error> {
    let state = app_state::State {
        ..Default::default()
    };

    scan_wifi();
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(http_server::core_server());
    // run the UI code and the IPC (internal Procedure Call functions)
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_wifi_hotspot,
            kill_wifi_hotspot,
            generate_qr_code,
            get_ip_address,
            get_system_information,
            get_transfer_history,
            get_settings,
            update_settings,
            read_dir,
            scan_wifi // download_file, TODO: implement file transfering between peers
        ])
        .run(tauri::generate_context!())
}
