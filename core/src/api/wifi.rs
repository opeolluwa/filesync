/// create network interface here using OS specific implementation
/// once this is done pass the network conf
use crate::{
    utils::CommandData,
    wifi::{hotspot, network_scanner, WifiHotspotConfig},
};

#[tauri::command]
pub fn create_wifi_hotspot() -> CommandData<WifiHotspotConfig> {
    #[cfg(target_os = "linux")]
    {
        // Linux-specific command
        let Some(new_access_point) = hotspot::linux::create_hotspot().ok() else {
            return CommandData::err("failed to create access point", WifiHotspotConfig::err());
        };
        CommandData::ok("created access point", new_access_point)
    }

    #[cfg(target_os = "windows")]
    {
        todo!();
    }

    #[cfg(target_os = "macos")]
    {
        todo!()
    }
}

// turn off wifi hotspot
#[tauri::command]
pub fn kill_wifi_hotspot() {
    #[cfg(target_os = "linux")]
    {
        hotspot::linux::turn_off_hotspot()
    }

    #[cfg(target_os = "windows")]
    {
        todo!();
    }

    #[cfg(target_os = "macos")]
    {
        todo!();
    }
}

// scan for available network
#[tauri::command]
pub fn scan_wifi() {
    #[cfg(target_os = "linux")]
    {
        network_scanner::linux::scan_wifi();
    }

    #[cfg(target_os = "windows")]
    {
        network_scanner::windows::scan_wifi();
    }

    #[cfg(target_os = "macos")]
    {
        network_scanner::mac::scan_wifi();
    }
}
