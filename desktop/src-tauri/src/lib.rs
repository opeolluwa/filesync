// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod database;
mod error;
mod utils;

use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_transfer_history_table",
        sql: "CREATE TABLE transfer_history (id TEXT PRIMARY KEY, file_name TEXT, sender TEXT, receiver TEXT,  file_size VARCHAR, date TEXT, status VARCHAR );",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:filesync.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::app::get_app_config,
            commands::keygen::generate_android_wifi_credentials
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
