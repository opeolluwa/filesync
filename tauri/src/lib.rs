// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;

use lazy_static::lazy_static;
use server::http_server;

mod commands;
mod database;
mod websockets;

mod server;
mod state;
mod utils;

mod pkg;

lazy_static! {

    pub static ref SERVER_PORT: u16 = 18005;
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("filesync");


    //create wi-share directory in the downloads path dir and / save files to $DOWNLOADS/filesync
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
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = state::State {
        ..Default::default()
    };

    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(http_server::core_server());

    // run the UI code and the IPC (internal Procedure Call functions)
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::files::get_transfer_history,
            commands::files::persist_transfer_history,
            commands::files::read_dir,
            commands::files::save_file_transfer,
            commands::files::share_file_with_peer,
            commands::network::broadcast_wifi,
            commands::network::connect_to_wifi,
            commands::network::get_available_wifi,
            commands::settings::get_application_data,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::utils::generate_qr_code,
            commands::utils::get_ip_address,
            commands::utils::get_system_information,
            commands::utils::is_connected_to_wifi,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
