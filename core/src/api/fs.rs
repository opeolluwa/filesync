use std::path::PathBuf;
use std::net::Ipv4Addr;

use crate::{
    fs::file::{get_files_in_directory, File},
    utils::{ApiResponse, CommandData},
};
use dirs;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ts_rs::TS;

use crate::database::{self, TransferHistory, TransferHistoryBuilder};
use crate::fs::search::search_files;
use crate::wifi::ip_manager;
use tokio::io::AsyncReadExt;

/// the dir enum, reads, $HOME, $PICTURES, $VIDEOS, $DOCUMENTS, $DOWNLOADS, and // Other
/// the other is a unit struct that contains a path of the directory to be red
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub(crate) enum Dir {
    Home,
    Pictures,
    Videos,
    Documents,
    Downloads,
    Audio,
    Desktop,
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
            _ => Dir::Other(path.to_string()),
        }
    }
}
/// read directory
#[tauri::command]
pub async fn read_dir(path: &str) -> ApiResponse<Vec<File>, ()> {
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

// #[tauri::command]
// pub fn search_home_dir(pattern: &str) -> Result<CommandData<Vec<File>>, CommandData<()>> {
//     let home_dir = dirs::home_dir();
//     let Some(home_dir) = home_dir else {
//         return Err(CommandData::err("error getting the home dir", ()));
//     };

//     let entries = search_files(pattern, &home_dir);

//     Ok(CommandData::ok(
//         "searched all files in home directory",
//         entries,
//     ))
// }

// send file from this server to another
// accept path to file as argument
// validate the file existence
// use streams to upload
// the server id is the port on which the peer node run eg -> 23345
#[tauri::command(async)]
pub async fn share_file_with_peer(
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
pub async fn persist_transfer_history(
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
