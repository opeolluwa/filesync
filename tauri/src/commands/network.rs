#[allow(unused_imports)]
use local_ip_address::local_ip;

#[cfg(not(target_os = "android"))]
use livewire::{WifiHotspotConfig, WifiHotspotConfigBuilder};

use crate::pkg::CommandData;
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#[cfg(target_family = "unix")]
extern crate pnet_datalink;
extern crate std;

#[cfg(target_os = "linux")]
pub fn autodetect_ip_address() -> Result<String, ()> {
    pnet_datalink::interfaces()
        .into_iter()
        .filter(|iface| !iface.is_loopback() && iface.is_up())
        .flat_map(|iface| iface.ips)
        .find(|ip_network| ip_network.is_ipv4())
        .map(|ip_network| ip_network.ip().to_string())
        .ok_or(())
}

#[cfg(target_os = "macos")]
pub fn autodetect_ip_address() -> Result<String, ()> {
    pnet_datalink::interfaces()
        .into_iter()
        .filter(|iface| !iface.is_loopback() && iface.is_up())
        .flat_map(|iface| iface.ips)
        .find(|ip_network| ip_network.is_ipv4())
        .map(|ip_network| ip_network.ip().to_string())
        .ok_or(())
}

#[cfg(target_os = "windows")]
pub fn autodetect_ip_address() -> Result<String, ()> {
    match local_ip() {
        Ok(ip) => Ok(ip.to_string()),
        Err(_) => Err(()),
    }
}

//TODO: implfor android a
#[cfg(target_os = "android")]
pub fn autodetect_ip_address() -> Result<String, ()> {
    let ip = "0.0.0.0.0";
    Ok(ip.to_string())
}

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
