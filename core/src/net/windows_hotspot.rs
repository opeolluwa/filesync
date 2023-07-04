use std::process::Command;
fn command_output_to_string(output: &Vec<u8>) -> String {
    let _output = std::str::from_utf8(&output);
    match _output {
        Ok(s) => s.to_string(),
        Err(_) => "None".to_string(),
    }
}
/// Create hotspot on windows
#[allow(dead_code)]
pub fn create_ap() {
    // Check if the device support the hotspot feature
    match is_support_hotspot() {
        Ok(false) => {
            println!("Hotspot is not supported");
            return;
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(true) => println!("Hotspot is supported"),
    };
    // Set up the hotspot name and password
    let hotspot_name = "MyHotspot";
    let hotspot_password = "MyPassword123";

    // Create a new hotspot
    match create_hotspot(hotspot_name, hotspot_password) {
        Ok(_) => println!("Created a new hotspot"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Start the hotspot
    match start_hotspot() {
        Ok(_) => println!("Started a new hotspot"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

#[warn(dead_code)]
pub fn turn_off_hotspot() {
    match stop_hotspot() {
        Ok(_) => println!("Stop the hotspot successfully"),
        Err(e) => println!("{}", e),
    }
}

fn is_support_hotspot() -> Result<bool, String> {
    match Command::new("netsh")
        .args(["wlan", "show", "drivers"])
        .output()
    {
        Ok(output) => {
            if command_output_to_string(&output.stdout)
                .as_str()
                .contains("Hosted network supported  : Yes")
            {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Err(format!("Failed to get the wlan drivers information")),
    }
}

fn create_hotspot(ssid: &str, key: &str) -> Result<(), String> {
    match Command::new("netsh")
        .args([
            "wlan",
            "set",
            "hostednetwork",
            &("ssid=".to_owned() + ssid),
            &("key=".to_owned() + key),
        ])
        .output()
    {
        Ok(output) => {
            if let true = output.status.success() {
                return Ok(());
            } else {
                return Err(format!(
                    "Failed to create hotspot: {}",
                    command_output_to_string(&output.stderr)
                ));
            }
        }
        Err(_) => return Err(format!("Failed to execute create hotspot through netsh.")),
    }
}

fn start_hotspot() -> Result<(), String> {
    match Command::new("netsh")
        .args(["wlan", "start", "hostednetwork"])
        .output()
    {
        Ok(output) => {
            if command_output_to_string(&output.stdout).contains("The hosted network started.") {
                return Ok(());
            } else {
                return Err(format!("Failed to start the hosted network"));
            }
        }
        Err(_) => return Err(format!("Failed to start the hosted network through netsh")),
    }
}

fn stop_hotspot() -> Result<(), String> {
    match Command::new("netsh")
        .args(["wlan", "stop", "hostednetwork"])
        .output()
    {
        Ok(output) => {
            if command_output_to_string(&output.stdout).contains("The hosted network stoped.") {
                return Ok(());
            } else {
                return Err(format!("Failed to stop the hosted network"));
            }
        }
        Err(_) => return Err(format!("Failed to stop the hosted network through netsh")),
    }
}
