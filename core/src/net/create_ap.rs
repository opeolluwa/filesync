/// create network interface here using OS specific implementation
/// once this is done pass the network conf
use super::{linux_hotspot, AccessPointInterface};
use crate::utils::CommandData;

// #[tauri::command]
// #[cfg(target_os = "windows")]
// pub fn create_ap() -> CommandData<AccessPointInterface> {
//     windows_hotspot::create_ap()
// }

// // create wifi hotspot on linux operating system and pass the configuration to the UI
// #[tauri::command(linux)]
// #[cfg(target_os = "linux")]
// pub fn create_ap() -> CommandData<AccessPointInterface> {
//     let Some(new_access_point) = linux_hotspot::create_ap().ok() else {
//         return CommandData::err("failed to create access point", AccessPointInterface::err())
//     };
//     CommandData::ok("created access point", new_access_point)
// }

#[tauri::command]
pub fn create_ap() -> CommandData<AccessPointInterface> {
    #[cfg(target_os = "linux")]
    {
        // Linux-specific command
        let Some(new_access_point) = linux_hotspot::create_ap().ok() else {
        return CommandData::err("failed to create access point", AccessPointInterface::err())
    };
        CommandData::ok("created access point", new_access_point)
    }

    #[cfg(target_os = "windows")]
    {
        // Windows-specific command
        todo!();
    }

    #[cfg(target_os = "macos")]
    {
        // macOS-specific command
        todo!();
        /*  tauri::Command::new("open")
        .arg("https://www.example.com")
        .spawn()
        .expect("Failed to execute command"); */
    }
}
