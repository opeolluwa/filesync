use std::collections::HashMap;

use crate::DB_URL;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteQueryResult, FromRow, Pool, Row, Sqlite, SqlitePool,
};
use ts_rs::TS;
use uuid::Uuid;

pub struct Database;
#[allow(unused)]

impl Database {
    /*
    initialize the database connection
    this will create a new sqlite database in the OS $Downloads/filesync/.filesync directory
     */
    pub async fn init() {
        if !Sqlite::database_exists(&DB_URL).await.unwrap_or(false) {
            match Sqlite::create_database(&DB_URL).await {
                Ok(_) => println!("Database initialized"),
                Err(_error) => eprintln!("error creating utility store"),
            }
        }

        // create the file history table
        let file_history_table =
            "CREATE TABLE IF NOT EXISTS transfer_history ( id VARCHAR PRIMARY KEY, file_name VARCHAR, file_size VARCHAR, transaction_type VARCHAR, date TEXT, recipient VARCHAR)";

        /* create the settings table,  the table will contain user preference and settings */
        //TODO:  add device name , device_name VARCHAR
        let settings_table =
            "CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY DEFAULT 1, language VARCHAR, theme VARCHAR)";

        let db = SqlitePool::connect(&DB_URL).await.unwrap();
        let _ = sqlx::query(file_history_table).execute(&db).await.unwrap();
        let _ = sqlx::query(settings_table).execute(&db).await.unwrap();
    }

    // return connection to the database;
    pub async fn conn() -> Pool<Sqlite> {
        SqlitePool::connect(&DB_URL).await.unwrap()
    }

    // get the tables in the database
    pub async fn tables() -> HashMap<usize, String> {
        let db = Self::conn().await;
        let result: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
        )
        .fetch_all(&db)
        .await
        .unwrap();

        let mut tables: HashMap<usize, String> = HashMap::new();
        for (idx, row) in result.iter().enumerate() {
            let key = idx;
            let value = row.get::<String, &str>("name");
            tables.insert(key, value.to_owned());
        }

        tables
    }

    // flush the database, this action remove all data from the database tables  and reset the database
    pub async fn flush() -> Result<SqliteQueryResult, ()> {
        let db = Self::conn().await;
        let _ = sqlx::query("DELETE FROM transfer_history")
            .execute(&db)
            .await
            .unwrap();
        let _ = sqlx::query("DELETE FROM settings")
            .execute(&db)
            .await
            .unwrap();
        let _ = sqlx::query("VACUUM").execute(&db).await.unwrap();
        Ok(sqlx::query("VACUUM").execute(&db).await.unwrap())
    }
}

/// file transfer history
#[derive(Debug, Clone, Serialize, FromRow, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TransferHistory {
    pub id: String,
    pub file_name: String,
    pub file_size: String,
    pub date: String,
    pub transaction_type: String, // sent or received
    pub recipient: String,
}

/// file transfer builder to be passed to TransferHistory::new()
#[derive(Debug, Clone, Serialize, FromRow, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TransferHistoryBuilder {
    file_name: String,
    file_size: String,
    transaction_type: String,
    recipient: String,
}

// impl display for history builder
impl std::fmt::Display for TransferHistoryBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let history = format!(
            "file_name: {}, file_size: {}, transaction_type: {}, recipient: {}",
            self.file_name, self.file_size, self.transaction_type, self.recipient
        );
        write!(f, "{}", history)
    }
}

// implement default and display for TransferHistory
impl Default for TransferHistory {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            file_name: "".to_string(),
            file_size: "".to_string(),
            date: Local::now()
                .date_naive()
                .format("%a, %-d %b, %C%y")
                .to_string(),
            transaction_type: "".to_string(),
            recipient: "".to_string(),
        }
    }
}

// impl display for TransferHistory
impl std::fmt::Display for TransferHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let history = format!(
            "id: {}, file_name: {}, file_size: {}, date: {}, transaction_type: {}, recipient: {}",
            self.id,
            self.file_name,
            self.file_size,
            self.date,
            self.transaction_type,
            self.recipient
        );
        write!(f, "{}", history)
    }
}
/// settings table
#[derive(Debug, Clone, Serialize, FromRow, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Settings {
    pub id: i32,
    pub language: String,
    pub theme: String,
    pub first_run: bool,
}

// implement default and display for Settings
impl Default for Settings {
    fn default() -> Self {
        Self {
            id: 1,
            language: "English".to_string(),
            theme: "Dark".to_string(),
            first_run: true,
        }
    }
}

// impl display for Settings
impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let settings = format!(
            "id: {}, language: {}, theme: {}, first_run: {}",
            self.id, self.language, self.theme, self.first_run
        );
        write!(f, "{}", settings)
    }
}

impl Settings {
    pub async fn save(&self) -> Result<Self, ()> {
        let db = Database::conn().await;
        let _ = sqlx::query(
            "INSERT OR REPLACE INTO settings (id, language, theme, first_run) VALUES (?,?,?,?)",
        )
        .bind(self.id)
        .bind(self.language.clone())
        .bind(self.theme.clone())
        .bind(self.first_run)
        .execute(&db)
        .await
        .unwrap();

        Ok(Self { ..self.clone() })
    }

    pub async fn fetch() -> Self {
        let db = Database::conn().await;
        let result = sqlx::query_as::<_, Self>("SELECT * FROM settings")
            .fetch_one(&db)
            .await
            .ok();

        if result.is_none() {
            let settings = Settings::default();
            settings.save().await.unwrap();
            return settings;
        }

        result.unwrap()
    }
}

#[allow(dead_code)]
/// impl file transfer history
impl TransferHistory {
    // impl new
    pub fn new(file: TransferHistoryBuilder) -> Self {
        let TransferHistoryBuilder {
            file_name,
            file_size,
            transaction_type,
            recipient,
        } = file;
        Self {
            file_name,
            file_size,
            transaction_type,
            recipient,
            ..Default::default()
        }
    }
    pub async fn save(&self) -> Result<Self, ()> {
        let db = Database::conn().await;
        let _ = sqlx::query(
            "INSERT INTO transfer_history (id, file_name, file_size, date, transaction_type, recipient) VALUES (?,?,?,?,?,?)",
        )
        .bind(self.id.clone())
        .bind(self.file_name.clone())
        .bind(self.file_size.clone())
        .bind(self.date.clone())
        .bind(self.transaction_type.clone())
        .bind(self.recipient.clone())
        .execute(&db)
        .await
        .unwrap();

        Ok(Self { ..self.clone() })
    }

    pub async fn fetch() -> Result<Vec<Self>, ()> {
        let db = Database::conn().await;
        let result = sqlx::query_as::<_, Self>("SELECT * FROM transfer_history")
            .fetch_all(&db)
            .await
            .ok();

        if result.is_none() {
            return Err(());
        }

        Ok(result.unwrap())
    }

    // delete history
    pub async fn delete(id: &str) {
        let db = Database::conn().await;
        let _ = sqlx::query_as::<_, Self>("DELETE FROM transfer_history WHERE id = ? ")
            .bind(id)
            .fetch_all(&db)
            .await
            .unwrap();
    }
}
