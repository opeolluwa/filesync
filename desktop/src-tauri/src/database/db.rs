use std::{env, fs};

use sqlx::{Pool, Sqlite};
use tauri::AppHandle;

use crate::error::StartupError;

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self, StartupError> {
        let app_dir = app_handle
            .path_resolver()
            .app_data_dir()
            .expect("failed to get app dir");

        // Ensure the app directory exists
   fs::create_dir_all(&app_dir)
            .map_err(|error| StartupError::ProcessFailed(error.to_string()))?;

        let db_path = app_dir.join("grid_search.db");

        // Set the DATABASE_URL environment variable to point to this SQLite file
        env::set_var("DATABASE_URL", format!("sqlite://{}", db_path.display()));

        let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
        todo!()
    }
}
