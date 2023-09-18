use mockall::predicate::*;
use mockall::*;
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
pub fn create_ap(ssid: &str, key: &str) {
    let hotspotcommand = DefaultHotSpotCommand;
    create_ap_with_hotspotcommand(hotspotcommand, ssid, key);
}
fn create_ap_with_hotspotcommand<T: HotSpotCommand>(hotspotcommand: T, ssid: &str, key: &str) {
    // Check if the device support the hotspot feature
    match hotspotcommand.is_support_hotspot() {
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
    // Create a new hotspot
    match hotspotcommand.create_hotspot(ssid, key) {
        Ok(_) => println!("Created a new hotspot"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Start the hotspot
    match hotspotcommand.start_hotspot() {
        Ok(_) => println!("Started a new hotspot"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

/// Turnoff the hotspot
#[warn(dead_code)]
pub fn turn_off_hotspot() {
    let hotspotcommand = DefaultHotSpotCommand;
    turn_off_hotspot_with_hotspotcommand(hotspotcommand);
}
fn turn_off_hotspot_with_hotspotcommand<T: HotSpotCommand>(hotspotcommand: T) {
    match hotspotcommand.stop_hotspot() {
        Ok(_) => println!("Stop the hotspot successfully"),
        Err(e) => println!("{}", e),
    }
}

struct DefaultHotSpotCommand;
#[automock]
pub trait HotSpotCommand {
    fn is_support_hotspot(&self) -> Result<bool, String>;
    fn create_hotspot(&self, ssid: &str, key: &str) -> Result<(), String>;
    fn start_hotspot(&self) -> Result<(), String>;
    fn stop_hotspot(&self) -> Result<(), String>;
}

impl HotSpotCommand for DefaultHotSpotCommand {
    fn is_support_hotspot(&self) -> Result<bool, String> {
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

    fn create_hotspot(&self, ssid: &str, key: &str) -> Result<(), String> {
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

    fn start_hotspot(&self) -> Result<(), String> {
        match Command::new("netsh")
            .args(["wlan", "start", "hostednetwork"])
            .output()
        {
            Ok(output) => {
                if command_output_to_string(&output.stdout).contains("The hosted network started.")
                {
                    return Ok(());
                } else {
                    return Err(format!("Failed to start the hosted network"));
                }
            }
            Err(_) => return Err(format!("Failed to start the hosted network through netsh")),
        }
    }

    fn stop_hotspot(&self) -> Result<(), String> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::Sequence;

    #[test]
    fn test_create_ap_when_success() {
        let mut mock_hotspotcommand = MockHotSpotCommand::new();

        let ssid = "TestSSID";
        let key = "TestKey";
        let mut seq = Sequence::new();

        mock_hotspotcommand
            .expect_is_support_hotspot()
            .times(1)
            .returning(|| Ok(true))
            .in_sequence(&mut seq);

        mock_hotspotcommand
            .expect_create_hotspot()
            .with(eq(ssid), eq(key))
            .times(1)
            .returning(|_, _| Ok(()))
            .in_sequence(&mut seq);

        mock_hotspotcommand
            .expect_start_hotspot()
            .times(1)
            .returning(|| Ok(()))
            .in_sequence(&mut seq);

        create_ap_with_hotspotcommand(mock_hotspotcommand, ssid, key);
    }
    #[test]
    fn test_create_ap_failure_when_unsupported() {
        let mut mock_hotspotcommand = MockHotSpotCommand::new();
        let ssid = "TestSSID";
        let key = "TestKey";
        let mut seq = Sequence::new();

        mock_hotspotcommand
            .expect_is_support_hotspot()
            .times(1)
            .returning(|| Ok(false))
            .in_sequence(&mut seq);
        mock_hotspotcommand.expect_create_hotspot().times(0);
        mock_hotspotcommand.expect_start_hotspot().times(0);
        create_ap_with_hotspotcommand(mock_hotspotcommand, ssid, key);
    }
    #[test]
    fn test_create_ap_failure_when_create_hotspot_failed() {
        let mut mock_hotspotcommand = MockHotSpotCommand::new();
        let ssid = "TestSSID";
        let key = "TestKey";
        let mut seq = Sequence::new();

        mock_hotspotcommand
            .expect_is_support_hotspot()
            .times(1)
            .returning(|| Ok(true))
            .in_sequence(&mut seq);
        mock_hotspotcommand
            .expect_create_hotspot()
            .times(1)
            .returning(|_, _| Err(format!("Failed")))
            .in_sequence(&mut seq);
        mock_hotspotcommand.expect_start_hotspot().times(0);
        create_ap_with_hotspotcommand(mock_hotspotcommand, ssid, key);
    }
    #[test]
    fn test_create_ap_failure_when_start_hotspot_failed() {
        let mut mock_hotspotcommand = MockHotSpotCommand::new();
        let ssid = "TestSSID";
        let key = "TestKey";
        let mut seq = Sequence::new();

        mock_hotspotcommand
            .expect_is_support_hotspot()
            .times(1)
            .returning(|| Ok(true))
            .in_sequence(&mut seq);
        mock_hotspotcommand
            .expect_create_hotspot()
            .times(1)
            .returning(|_, _| Ok(()))
            .in_sequence(&mut seq);
        mock_hotspotcommand
            .expect_start_hotspot()
            .times(1)
            .returning(|| Err(format!("Failed")))
            .in_sequence(&mut seq);
        create_ap_with_hotspotcommand(mock_hotspotcommand, ssid, key);
    }

    #[test]
    fn test_turn_off_hotspot_success() {
        let mut mock_hotspotcommand = MockHotSpotCommand::new();

        mock_hotspotcommand
            .expect_stop_hotspot()
            .times(1)
            .returning(|| Ok(()));

        turn_off_hotspot_with_hotspotcommand(mock_hotspotcommand);
    }
    #[test]
    fn test_turn_off_hotspot_failure_when_stop_hotspot_failed() {
        let mut mock_hotspotcommand = MockHotSpotCommand::new();
        mock_hotspotcommand
            .expect_stop_hotspot()
            .times(1)
            .returning(|| Err(format!("Failed")));
        turn_off_hotspot_with_hotspotcommand(mock_hotspotcommand);
    }
}
