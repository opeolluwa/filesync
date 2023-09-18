// manage the application state
#[allow(dead_code)]

use crate::database::{Settings, TransferHistory};

pub struct State {
    pub settings: Settings,
    pub transfer_history: Vec<TransferHistory>,
}

impl State {
    pub async fn fetch() -> Self {
        let settings = Settings::fetch().await;
        let transfer_history = TransferHistory::fetch().await;

        Self {
            settings,
            transfer_history,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            transfer_history: Vec::new(),
        }
    }
}
