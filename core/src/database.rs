use std::{collections::HashMap, fmt::Display};

use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteQueryResult, FromRow, Pool, Row, Sqlite, SqlitePool};
use uuid::Uuid;

use crate::DB_URL;
pub struct Database(pub Vec<Store>);
impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_fold((), |_, data| write!(f, "{}", data))
    }
}
#[allow(unused)]

impl Database {
    /*
    initialize the database connection
    this will create a new sqlite database in the OS home directory
     */
    pub async fn init() {
        if !Sqlite::database_exists(&DB_URL).await.unwrap_or(false) {
            match Sqlite::create_database(&DB_URL).await {
                Ok(_) => PrintColoredText::success("Database initialized"),
                Err(_error) => PrintColoredText::error("error creating utility store"),
            }
        }

        // create the file history table
        let file_history_table =
            "CREATE TABLE IF NOT EXISTS transfer_history ( id VARCHAR PRIMARY KEY, file_name VARCHAR, file_size VARCHAR, transaction_type VARCHAR date TEXT, recipient VARCHAR)";

        let settings_table =
            "CREATE TABLE IF NOT EXISTS settings ( id INTEGER PRIMARY KEY DEFAULT 1, language VARCHAR, theme VARCHAR, first_run BOOLEAN)";

        let db = SqlitePool::connect(&DB_URL).await.unwrap();
        let _ = sqlx::query(file_history_table).execute(&db).await.unwrap();
        let _ = sqlx::query(settinsg_table).execute(&db).await.unwrap();
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

    // flush the databse
    pub async fn flush() {
}

/// file transfer history
#[derive(Debug, Clone, Serialize, FromRow, Deserialize)]
pub struct TransferHistory {
    pub id: String,
    pub file_name: String,
    pub file_size: String,
    pub date: String,
    pub transaction_type: TransactionType,
    pub recipient: String,
}

enum TransactionType {
    Sent,
    Received,
}

/// settings table
#[derive(Debug, Clone, Serialize, FromRow, Deserialize)]
pub struct Settings {
    pub id: i32,
    pub language: String,
    pub theme: String,
    pub first_run: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: 1,
            language: "en".to_string(),
            theme: "light".to_string(),
            first_run: true,
        }
    }
}

/// for deserializing data from the database
#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct Store {
    pub id: String,
    pub key: String,
    pub value: String,
    pub date_added: String,
    pub last_updated: String,
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

/// impl file transfer history
impl TransferHistory {
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

    pub async fn fetch() -> Vec<Self> {
        let db = Database::conn().await;

        sqlx::query_as::<_, Self>("SELECT * FROM transfer_history")
            .fetch_all(&db)
            .await
            .unwrap()
    }

    // delete history
    pub async fn delete(id: &str) {
        let db = Database::conn().await;
        let _ = sqlx::query_as::<_, Self>("DELETE FROM transfer_history WHERE id = ?")
            .bind(id)
            .fetch_all(&db)
            .await
            .unwrap();
        // let message = format!("{id} removed successfully");
    }
}



