use livewire::{WifiHotspotConfig, WifiHotspotConfigBuilder};

use crate::utils::CommandData;

///scan for available network
#[tauri::command]
pub async fn get_available_wifi() -> CommandData<Vec<String>> {
    let networks = WifiHotspotConfig::scan()
        .unwrap()
        .into_iter()
        .map(|network| network.ssid)
        .collect::<Vec<_>>();

    CommandData::new(networks)
}


/// boardcast - crate wifi hotstop
/// returns the network name
#[tauri::command]
pub async fn broadcast_wifi() -> CommandData<String> {
    let ssid = "test".to_string();
    CommandData::new(ssid)
}


/// connect to wifi
#[tauri::command]
pub async fn connect_to_wifi(ssid: String) -> CommandData<bool> {
    CommandData::new(true)
}
