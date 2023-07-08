use super::WifiHotspotConfig;
use crate::net::NetworkAccessStatus;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Output, Stdio};
use std::str;
/// create hotspot on linux
pub fn create_hotspot() -> Result<WifiHotspotConfig, WifiHotspotConfig> {
    // get the network gate way ex DNS Configuration: Some(["192.168.100.121"])
    let output = Command::new("nmcli")
        .args(["dev", "show"])
        .output()
        .expect("Failed to execute nmcli");

    let Some(network_gateway) = parse_dns_config(&output) else {
        return Err(WifiHotspotConfig{
            status: Some(NetworkAccessStatus::Error),
            message: Some(String::from("Failed to create Wifi hotspot")),
            ..Default::default()
        });
    };
    println!("network gateway {:#?}", network_gateway);

    // create new access point config
    let access_point = WifiHotspotConfig::new(&network_gateway[0]);
    // destructure the ssid, password, and gateway
    let WifiHotspotConfig {
        ssid,
        password,
        gateway,
        ..
    } = access_point;

    // refresh virtual access card interface. !! Do not remove
    let _ = execute_shell_command("nmcli radio wifi off && nmcli radio wifi on");
    // get the network interface e.g wlan0, wlo1 ...
    let Some(network_interface) = execute_shell_command("ls /sys/class/net/ | grep \"^wl.\\+\"").ok() else {
            return Err(WifiHotspotConfig {
                status: Some(NetworkAccessStatus::Error),
                message: Some(String::from("Wifi Hotspot not supported")),
                ..Default::default()
            });
        };

    println!("interface {}", network_interface);

    // Execute 'nmcli' commands to create a hotspot
    let create_wifi_command = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("hotspot")
        .arg("ifname")
        .arg(&network_interface) // Replace 'wlan0' with the appropriate network interface name
        .arg("con-name")
        .arg("wishare") // Replace 'my-hotspot' with the desired connection name
        .arg("ssid")
        .arg(&ssid) //the desired SSID name
        .arg("password")
        .arg(&password) // Replace 'MyPassword' with the desired password
        .output()
        .expect("Failed to execute 'nmcli' command."); //nmcli dev wifi hotspot ifname wlo1 con-name wishare ssid ghost password 1234test1234

    // Check if the command was successful
    if create_wifi_command.status.success() {
        return Ok(WifiHotspotConfig {
            ssid,
            password,
            gateway,
            status: Some(NetworkAccessStatus::Created),
            message: Some(String::from("Wifi hotspot created successfully")),
        });
        // break;
    } else {
        let error_msg = String::from_utf8_lossy(&create_wifi_command.stderr);
        return Err(WifiHotspotConfig {
            status: Some(NetworkAccessStatus::Error),
            message: Some(format!("Failed to create hotspot: {}", error_msg)),
            ..Default::default()
        });
    }
}

/// get the network gate way DNS Configuration: Some(["192.168.100.121"])
fn parse_dns_config(output: &Output) -> Option<Vec<String>> {
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
    let mut dns_config = Vec::new();

    for line in stdout.lines() {
        // println!("{:?}",line);
        if line.contains("IP4.DNS") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(dns_value) = parts.get(1) {
                dns_config.push(dns_value.to_string());
            }
        }
    }

    if dns_config.is_empty() {
        None
    } else {
        Some(dns_config) // DNS Configuration: Some(["192.168.100.121"])
    }
}

/// get the network interface
fn get_network_interface() -> std::io::Result<Vec<String>> {
    let output = Command::new("nmcli").args(["device", "show"]).output()?;

    let mut network_interfaces: Vec<String> = Vec::new();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            if line.contains("GENERAL.DEVICE")
            /* && line.contains("TYPE") */
            {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if let Some(interface) = parts.get(1) {
                    // return Ok(interface.to_string());
                    network_interfaces.push(interface.to_string());
                }
            }
        }
        return Ok(network_interfaces); // [wlo1, wlan0, docker0 ...]
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing nmcli: {}", stderr);
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to get network interface",
    ))
}

// turn off the hotpot and refresh the virrtual Access card
pub fn turn_off_hotspot() {
    let _ = execute_shell_command("nmcli radio wifi off && nmcli radio wifi on");
}

/// execute shell command
/// accept raw string and execur the command using bash, zsh, etc
/// #example
/// ```rust
///   match execute_shell_command("ls /sys/class/net/ | grep \"^wl.\\+\"") {
///       Ok(output) => println!("Command output:\n{}", output),
///    Err(err) => eprintln!("Error executing command: {}", err),
///   }
/// ```
fn execute_shell_command(command: &str) -> io::Result<String> {
    let mut cmd = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdout = cmd.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let output = reader
        .lines()
        .collect::<io::Result<Vec<String>>>()?
        .join("\n");

    let status = cmd.wait()?;

    if status.success() {
        Ok(output)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command '{}' failed with exit code: {}", command, status),
        ))
    }
}
