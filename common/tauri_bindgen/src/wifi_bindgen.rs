use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WifiCredentials {
    pub ssid: u16,
    pub passkey: String,
}

impl Display for WifiCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.ssid, self.passkey)
    }
}
