use serde::{Deserialize, Serialize};
use std::fmt::{self};
use sysinfo::{Components, Disks, Networks, System};
use ts_rs::TS;

/// detect if the application is running ona  mobile platform or not
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]

pub struct DevicePlatform {
    pub operating_system: String,
    pub is_mobile: bool,
}

impl DevicePlatform {
    pub fn new() {
        let mut sys = System::new_all();
        sys.refresh_all();
    }
}
