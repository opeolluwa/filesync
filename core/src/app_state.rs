// manage the application state
#[allow(dead_code)]
use crate::database::{Settings, TransferHistory};

#[derive(Default)]
pub struct State {
    pub settings: Settings,
    pub transfer_history: Vec<TransferHistory>,
}

impl State {
    pub async fn _fetch() -> Self {
        let settings = Settings::fetch().await;
        let transfer_history = TransferHistory::fetch().await.unwrap();

        Self {
            settings,
            transfer_history,
        }
    }
}


