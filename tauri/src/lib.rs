// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;
use server::http_server::HttpServer;
use tauri::Emitter;

mod commands;
mod database;

mod config;
mod state;
mod utils;

mod pkg;

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
 * tauri::Builder::default().plugin(tauri_plugin_single_instance::init()).plugin(tauri_plugin_barcode_scanner::init()).plugin(tauri_plugin_os::init()).plugin(tauri_plugin_shell::init()).plugin(tauri_plugin_fs::init()).plugin(tauri_plugin_dialog::init()).plugin(tauri_plugin_clipboard_manager::init()).run().await // run the ui process
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
    tauri::async_runtime::spawn(HttpServer::run());
    //    .plugin(tauri_plugin_barcode_scanner::init())≈

    
    // run the UI code and the IPC (internal Procedure Call functions)
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            app.emit("single-instance", ()).unwrap();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_system_info::init())
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
            commands::device::get_device_information
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
