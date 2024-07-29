use serde::{Deserialize, Serialize};
use ts_rs::TS;

// manage the application state
#[allow(dead_code)]
use crate::database::{Settings, TransferHistory};
use crate::utils::system_info::SystemInformation;

#[derive(Default, Debug, Serialize, Deserialize, TS)]
#[ts(export)]

pub struct State {
    pub settings: Settings,
    pub transfer_history: Vec<TransferHistory>,
    pub system_information: SystemInformation,
}

impl State {
    pub async fn collect() -> Self {
        let settings = Settings::fetch().await;
        let transfer_history = TransferHistory::fetch().await.unwrap();
        let system_information = SystemInformation::new();

        Self {
            settings,
            transfer_history,
            system_information,
        }
    }
}
