#[cfg(not(target_os = "android"))]
use livewire::{WifiHotspotConfig, WifiHotspotConfigBuilder};

use pkg::CommandData;

///scan for available network
#[tauri::command]
#[cfg(not(target_os = "android"))]
pub async fn get_available_wifi() -> CommandData<Vec<String>> {
    let networks = WifiHotspotConfig::scan()
        .unwrap()
        .into_iter()
        .map(|network| network.ssid)
        .collect::<Vec<_>>();

    CommandData::new(networks)
}

#[tauri::command]
#[cfg(target_os = "android")]
pub async fn get_available_wifi() -> CommandData<Vec<String>> {
    todo!("handler has not been implemented for mobile ")
}

/// boroadcast - crate wifi hotstop
/// returns the network name
#[tauri::command]
#[cfg(not(target_os = "android"))]
pub async fn broadcast_wifi() -> CommandData<String> {
    let ssid = "test".to_string();
    CommandData::new(ssid)
}

#[tauri::command]
#[cfg(target_os = "android")]
pub async fn broadcast_wifi() -> CommandData<String> {
    unimplemented!("no support for mobile yet")
}

/// connect to wifi
#[tauri::command]
#[cfg(not(target_os = "android"))]
pub async fn connect_to_wifi(_ssid: String) -> CommandData<bool> {
    CommandData::new(true)
}

#[tauri::command]
#[cfg(target_os = "android")]
pub async fn connect_to_wifi(_ssid: String) -> CommandData<bool> {
    CommandData::new(true)
}
