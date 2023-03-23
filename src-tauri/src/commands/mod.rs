use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CommandData<T> {
    pub data: Option<T>,
    message: String,
    status: bool,
}

impl<T> Default for CommandData<T> {
    fn default() -> Self {
        Self {
            data: None::<T>,
            message: String::from("returned data form core"),
            status: true,
        }
    }
}

impl<T> CommandData<T> {
    fn new(message: &str, status: bool, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            status,
        }
    }
}
// get the ip address of the machine
#[tauri::command]
pub fn get_ip_addr() -> String {
    local_ip().unwrap().to_string()
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


//exports
pub mod audio;
pub mod documents;
pub mod image;
pub mod video;
