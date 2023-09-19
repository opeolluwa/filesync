use std::net::Ipv4Addr;

use serde_json::{json, Value};

use crate::database::{self, TransferHistory, TransferHistoryBuilder};
use crate::fs::file::File;
use crate::fs::search::search_files;
use crate::utils::CommandData;
use crate::wifi::ip_manager;
use tokio::io::AsyncReadExt;

pub mod audio;
pub mod document;
pub mod image;
pub mod video;

#[tauri::command]
pub fn search_home_dir(pattern: &str) -> Result<CommandData<Vec<File>>, CommandData<()>> {
    let home_dir = dirs::home_dir();
    let Some(home_dir) = home_dir else {
        return Err(CommandData::err("error getting the home dir", ()));
    };

    let entries = search_files(pattern, &home_dir);

    Ok(CommandData::ok(
        "searched all files in home directory",
        entries,
    ))
}

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
