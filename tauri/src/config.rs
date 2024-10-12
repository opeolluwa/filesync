use std::fmt::Display;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// where the files would be stored on the local system, it's the same as the application name
    pub upload_directory: String,
    /// port binding of the embedded file server
    pub server_port: u16,
    ///create wi-share directory in the downloads path dir and / save files to $DOWNLOADS/filesync
    pub upload_path: String,
    /// database url for the embedded Sqlite database
    pub db_url: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "upload_directory:{}\nserver_port:{}\nupload_path:{}\ndb_url:{}",
            self.upload_directory, self.server_port, self.upload_path, self.db_url
        )
    }
}

impl Config {
    pub fn parse() -> Self {
        let server_port = 18005u16;

        let upload_directory = String::from("filesync");

        let upload_path = {
            let os_default_downloads_dir = dirs::download_dir().unwrap();
            format!(
                "{downloads_dir}/{upload_dir}",
                downloads_dir = os_default_downloads_dir.display(),
                upload_dir = &upload_directory
            )
        };

        /* create a database in the home dir and / save files to $HOME/filesync/.dat */
        let db_url = {
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

        Self {
            upload_directory,
            server_port,
            upload_path,
            db_url,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::parse();
}
