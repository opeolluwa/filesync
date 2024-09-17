//! info about the device; OS, battery and memory
//!
//!

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::device_memory::DeviceMemory;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub os_type: String,
    pub memory: Value,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "os_type:{}\nmemory:{}", self.os_type, self.memory)
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}

impl Device {
    pub fn new() -> Self {
        let device_info = os_info::get();
        Self {
            os_type: device_info.os_type().to_string(),
            memory: DeviceMemory::new().to_readable(),
        }
    }
}

