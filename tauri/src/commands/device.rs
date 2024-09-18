use device::device_information::Device;

use crate::pkg::CommandData;

#[tauri::command]
pub fn get_device_information() -> CommandData<Device> {
    let device_information = Device::new();
    CommandData::new(device_information)
}
