//! Gives information about device memory
//!
//!

use std::fmt::Display;

use filesystem::directory::compute_file_size as convert_to_readable;
use serde::{Deserialize, Serialize};

use serde_json::Value;
use sysinfo::System;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceMemory {
    pub total_mem: u64,
    pub free_mem: u64,
}

impl Display for DeviceMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "total_memory:{}\nfree_memory:{}",
            self.total_mem, self.free_mem
        )
    }
}

impl DeviceMemory {
    pub fn new() -> Self {
        let system = System::new_all();

        Self {
            total_mem: system.total_memory(),
            free_mem: system.free_swap(),
        }
    }

    /// convert to human readable format 
    pub fn to_readable(&self) -> Value {
        serde_json::json!({
             "total_mem": convert_to_readable(self.total_mem as u128),
            "free_mem": convert_to_readable(self.free_mem as u128),
        })
    }
}



impl Default for DeviceMemory {
    fn default() -> Self {
        Self::new()
    }
}