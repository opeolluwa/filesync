//! Gives information about device memory
//!
//!

use std::fmt::Display;

use filesystem::directory::compute_file_size as convert_to_readable;
use serde::{Deserialize, Serialize};

use sysinfo::System;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceMemory {
    pub total_memory: u64,
    pub free_memory: u64,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ReadableDeviceMemory {
    pub total_memory: String,
    pub free_memory: String,
}

impl Display for ReadableDeviceMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "total_memory:{}\nfree_memory:{}",
            self.total_memory, self.free_memory
        )
    }
}

impl DeviceMemory {
    pub fn new() -> Self {
        let system = System::new_all();

        Self {
            total_memory: system.total_memory(),
            free_memory: system.free_swap(),
        }
    }

    /// convert to human readable format
    pub fn to_readable(&self) -> ReadableDeviceMemory {
        ReadableDeviceMemory {
            total_memory: convert_to_readable(self.total_memory as u128),
            free_memory: convert_to_readable(self.free_memory as u128),
        }
    }
}

impl Default for DeviceMemory {
    fn default() -> Self {
        Self::new()
    }
}
