use local_ip_address::local_ip;

// get the ip address of the machine
#[tauri::command]
pub fn get_ip_addr() -> String {
    local_ip().unwrap().to_string()
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
