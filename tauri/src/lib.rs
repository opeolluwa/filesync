// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use server::server::HttpServer;
use tauri_plugin_sql::{Migration, MigrationKind};

mod commands;

mod config;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database_migration = vec![
        Migration {
        version: 1,
        description: "create file transfer history table",
        sql: "CREATE TABLE IF NOT EXISTS transfer_history ( id VARCHAR PRIMARY KEY, file_name VARCHAR, file_size VARCHAR, transaction_type VARCHAR, date TEXT, recipient VARCHAR)" ,
        kind: MigrationKind::Up,
    },
    Migration {
        version: 2,
        description: "create application settings table",
        sql:  "CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY DEFAULT 1, language VARCHAR, theme VARCHAR)",
        kind: MigrationKind::Up,
    }
    ];

    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(HttpServer::run());

    // run the UI code and the IPC (internal Procedure Call functions)
    tauri::Builder::default()
        // .plugin(tauri_plugin_updater::Builder::new().build())
        // .plugin(tauri_plugin_sql::Builder::new().build())
        // .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // app.emit("single-instance", ()).unwrap();
        // }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        // .plugin(tauri_plugin_system_info::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:filesync.db", database_migration)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::files::read_dir,
            commands::files::save_file_transfer,
            commands::files::share_file_with_peer,
            commands::network::broadcast_wifi,
            commands::network::connect_to_wifi,
            commands::network::get_available_wifi,
            commands::utils::generate_qr_code,
            commands::utils::get_ip_address,
            commands::utils::get_system_information,
            commands::utils::is_connected_to_wifi,
            commands::device::get_device_information
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use serde::{Deserialize, Serialize};
use std::fmt::{self};
use ts_rs::TS;

/// data structure of response to return from Tauri Core
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommandData<T> {
    pub data: Option<T>,
    pub message: String,
    pub status: bool,
}

pub type ApiResponse<D, E> = Result<CommandData<D>, CommandData<E>>;

impl<T: fmt::Display + fmt::Debug> fmt::Display for CommandData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "data: {:?}
            message: {},
            status: {}
            ",
            self.data, self.message, self.status,
        )
    }
}

impl<T> Default for CommandData<T> {
    fn default() -> Self {
        Self {
            data: None::<T>,
            message: String::from("returned data form core"),
            status: true,
        }
    }
}

impl<T> CommandData<T> {
    /// only retun the data
    pub fn new(data: T) -> Self {
        Self {
            data: Some(data),
            ..Default::default()
        }
    }
    /// if the response is ok
    /// returns a CommandData struct
    /// with the data, message, and status
    pub fn ok(message: &str, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// if the response is an error
    /// returns a CommandData struct
    /// with the data, message, and status
    pub fn err(message: &str, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            status: false,
        }
    }
}
