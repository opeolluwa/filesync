use local_ip_address::local_ip;
// use reqwest::Response;
use serde_json::{json, Value};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use super::CommandData;

// send file from this server to another
// accept path to file as argument
// validate the file existence
// use streams to upload
// the server id is the port on which the peer node run eg -> 23345
#[tauri::command]
pub async fn share_file_with_peer(
    file_path: String,
    server_id: u16,
) -> Result<CommandData<Value>, CommandData<()>> {
    let mut file = File::open(file_path).await.unwrap();
    let mut vec = Vec::new();
    println!("file content {vec:?}");
    let _ = file.read_to_end(&mut vec).await.unwrap();
    // file.read_to_end(&mut vec).await.unwrap();
    let client = reqwest::Client::new();
    println!("yo! im in here");
    // get the IP address of the share network
    let my_local_ip = local_ip().unwrap();
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
    Ok(CommandData::new(
        "file successfully sent",
        true,
        json!({
            "success":true,
            // data:r
        }),
    ))
    // todo!()
}
