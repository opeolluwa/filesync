use local_ip_address::local_ip;
use reqwest::Response;
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
    server_id: u64,
) -> Result<CommandData<Response>, CommandData<()>> {
    let mut file = File::open(file_path).await.unwrap();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec);
    // file.read_to_end(&mut vec).await.unwrap();
    let client = reqwest::Client::new();

    // get the IP address of the share network
    let my_local_ip = local_ip().unwrap();
    let ip_address = format!("{:?}:{:?}", my_local_ip, server_id);
    let res = client
        .post(&ip_address)
        .header("content-type", "application/octet-stream")
        .body(vec)
        .send()
        .await
        .unwrap();

    // return an instance of the command data
    Ok(CommandData::new("file successfully sent", true, res))
}
