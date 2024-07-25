// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;
use crate::file_manager::get_transfer_history;
use crate::file_manager::read_dir;
use crate::ipc_manager::settings::{get_application_data, get_settings, update_settings};
use crate::ipc_manager::utils::{
    generate_qr_code, get_ip_address, get_system_information, is_connected_to_wifi,
};
use crate::network_manager::wifi_manager::{broadcast_wifi, connect_to_wifi, get_available_wifi};

use lazy_static::lazy_static;
use server::http_server;
mod context;
mod database;
mod file_manager;
mod ipc_manager;
mod network_manager;
mod server;
mod state_manager;
mod utils;

// pub const STATIC_ASSETS_DIRECTORY: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/views");

lazy_static! {
    /**
 * the lazy static crate allow the lazy evaluation of constants thus, one can bypass the impossible dynamic bindings of constants
 *
 *
 * Herein the server port made globally available, this allow for ease of sharing same with file upload directory
 */
    pub static ref SERVER_PORT: u16 = 18005;
        // portpicker::pick_unused_port().expect("failed to get an unused port");
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("filesync");


    // the static files directory


            //create wi-share directory in the downloads path dir and / save files to $DOWNLOADS/wi-share
     pub static ref UPLOAD_PATH  : std::string::String = {
           let os_default_downloads_dir = dirs::download_dir().unwrap();
      format!(
            "{downloads_dir}/{upload_dir}",
            downloads_dir = os_default_downloads_dir.display(),
            upload_dir = UPLOAD_DIRECTORY.as_str()
        )
     };

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
 * tauri::Builder::default().plugin(tauri_plugin_shell::init()).plugin(tauri_plugin_fs::init()).plugin(tauri_plugin_dialog::init()).plugin(tauri_plugin_clipboard_manager::init()).run().await // run the ui process
 *
 * ```
 *
 * That was the prev implementation but it doesn't work, either the server isn't up or the UI is not rendered
 *
 * thus the app was moved to run in separate thread
 */
// #[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() -> Result<(), tauri::Error> {
    let state = state_manager::State {
        ..Default::default()
    };

    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(http_server::core_server());

    // run the UI code and the IPC (internal Procedure Call functions)
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            generate_qr_code,
            get_ip_address,
            get_system_information,
            get_transfer_history,
            get_settings,
            is_connected_to_wifi,
            update_settings,
            read_dir,
            get_application_data,
            broadcast_wifi,
            connect_to_wifi,
            get_available_wifi
        ])
        .run(tauri::generate_context!())
}
