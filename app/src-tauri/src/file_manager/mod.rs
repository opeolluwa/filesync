pub mod file;

use crate::UPLOAD_PATH;
use crate::{
    file_manager::file::{get_files_in_directory, File},
    utils::{ApiResponse, CommandData},
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::Ipv4Addr;
use std::path::PathBuf;
use ts_rs::TS;

use crate::database::{self, TransferHistory, TransferHistoryBuilder};
use crate::network_manager::ip_manager;
use tokio::io::AsyncReadExt;

/// the dir enum, reads, $HOME, $PICTURES, $VIDEOS, $DOCUMENTS, $DOWNLOADS, and // Other
/// the other is a unit struct that contains a path of the directory to be red
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub(crate) enum Dir {
    /// the device home directory
    Home,
    Pictures,
    Videos,
    Documents,
    Downloads,
    Audio,
    Desktop,
    /// the location the files received are saved
    FileSync,
    Other(String),
}

impl Dir {
    /// to string
    pub fn _to_string(&self) -> String {
        match self {
            Dir::Home => dirs::home_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Pictures => dirs::picture_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Videos => dirs::video_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Documents => dirs::document_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Downloads => dirs::download_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Audio => dirs::audio_dir().unwrap().to_str().unwrap().to_string(),
            Dir::Desktop => dirs::desktop_dir().unwrap().to_str().unwrap().to_string(),
            Dir::FileSync => UPLOAD_PATH.to_string(),
            Dir::Other(path) => path.to_string(),
        }
    }

    // convert to path
    pub fn to_path(&self) -> std::path::PathBuf {
        match self {
            Dir::Home => dirs::home_dir().unwrap(),
            Dir::Pictures => dirs::picture_dir().unwrap(),
            Dir::Videos => dirs::video_dir().unwrap(),
            Dir::Documents => dirs::document_dir().unwrap(),
            Dir::Downloads => dirs::download_dir().unwrap(),
            Dir::Audio => dirs::audio_dir().unwrap(),
            Dir::Desktop => dirs::desktop_dir().unwrap(),
            Dir::FileSync => PathBuf::from(UPLOAD_PATH.to_string()),
            Dir::Other(path) => PathBuf::from(path),
        }
    }

    // from string
    pub fn from_string(path: &str) -> Self {
        match path {
            "home" => Dir::Home,
            "pictures" => Dir::Pictures,
            "videos" => Dir::Videos,
            "documents" => Dir::Documents,
            "downloads" => Dir::Downloads,
            "audio" => Dir::Audio,
            "desktop" => Dir::Desktop,
            "filesync" => Dir::FileSync,
            _ => Dir::Other(path.to_string()),
        }
    }
}

/// read directory
#[tauri::command]
pub async fn read_dir(path: &str) -> ApiResponse<Vec<File>, ()> {
    println!("reading from {path}");
    let path = Dir::from_string(path).to_path();
    let files = get_files_in_directory(&path).await;
    if files.is_err() {
        return Err(CommandData::err("Error fetching files", ()));
    }

    // convert to file type
    let mut entries: Vec<File> = Vec::new();
    for entry in files.unwrap() {
        let file_path = PathBuf::from(entry);
        let file = File::from_path(&file_path);
        entries.push(file)
    }

    Ok(CommandData::ok("Successfully fetch the data", entries))
}

// send file from this server to another
// accept path to file as argument
// validate the file existence
// use streams to upload
// the server id is the port on which the peer node run eg -> 23345
#[tauri::command(async)]
pub async fn _share_file_with_peer(
    file_path: String,
    server_id: u16,
) -> Result<CommandData<Value>, CommandData<()>> {
    let mut file = tokio::fs::File::open(file_path).await.unwrap();
    let mut vec = Vec::new();
    println!("file content {vec:?}");
    let _ = file.read_to_end(&mut vec).await.unwrap();
    // println!("file content {vec:?}");

    // file.read_to_end(&mut vec).await.unwrap();
    let client = reqwest::Client::new();

    // get the IP address of the share network
    let my_local_ip = ip_manager::autodetect_ip_address()
        .ok()
        .unwrap()
        // .expect("Invalid Ip address detected")
        .parse::<Ipv4Addr>()
        .unwrap();
    let ip_address = format!("http://{:?}:{:?}/upload", my_local_ip, server_id);

    println!("my client id is {ip_address}");
    let _res = client
        .post(&ip_address)
        .header("content-type", "application/octet-stream")
        .body(vec)
        .send()
        .await
        .unwrap();

    println!("the response here {_res:?}");

    // return an instance of the command data
    // Ok(CommandData::new("file successfully sent", true, res))
    Ok(CommandData::ok(
        "file successfully sent",
        json!({
            "success":true,
            // data:r
        }),
    ))
    // todo!()
}

// save file transfer to the database
//TODO: test this
#[tauri::command(async)]
pub async fn _save_file_transfer(
    file_path: String,
    server_id: u16,
) -> Result<CommandData<Value>, CommandData<()>> {
    let mut file = tokio::fs::File::open(file_path).await.unwrap();
    let mut vec = Vec::new();
    println!("file content {vec:?}");
    let _ = file.read_to_end(&mut vec).await.unwrap();
    // println!("file content {vec:?}");

    // file.read_to_end(&mut vec).await.unwrap();
    let client = reqwest::Client::new();

    // get the IP address of the share network
    let my_local_ip = ip_manager::autodetect_ip_address()
        .ok()
        .unwrap()
        // .expect("Invalid Ip address detected")
        .parse::<Ipv4Addr>()
        .unwrap();
    let ip_address = format!("http://{:?}:{:?}/upload", my_local_ip, server_id);

    println!("my client id is {ip_address}");
    let _res = client
        .post(&ip_address)
        .header("content-type", "application/octet-stream")
        .body(vec)
        .send()
        .await
        .unwrap();

    println!("the response here {_res:?}");

    // return an instance of the command data
    // Ok(CommandData::new("file successfully sent", true, res))
    Ok(CommandData::ok(
        "file successfully sent",
        json!({
            "success":true,
            // data:r
        }),
    ))
    // todo!()
}

// save file transfer history
#[tauri::command(async)]
pub async fn _persist_transfer_history(
    file: TransferHistoryBuilder,
) -> Result<CommandData<TransferHistory>, CommandData<()>> {
    // save the file data in the database
    let status = database::TransferHistory::new(file).save().await;
    if status.is_err() {
        return Err(CommandData::err("error saving file transfer history", ()));
    }

    Ok(CommandData::ok(
        "file transfer history successfully saved",
        status.unwrap(),
    ))
}

// get the file transfer history
#[tauri::command(async)]
pub async fn get_transfer_history() -> Result<CommandData<Vec<TransferHistory>>, CommandData<()>> {
    // save the file data in the database
    let data = database::TransferHistory::fetch().await;
    if data.is_err() {
        return Err(CommandData::err("error fetching file transfer history", ()));
    }

    Ok(CommandData::ok(
        "file transfer history successfully fetched",
        data.unwrap(),
    ))
}
