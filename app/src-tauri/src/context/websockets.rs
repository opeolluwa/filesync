use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct ClientMessage {
    pub payload: String,
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct ServerMessage {
    payload: String,
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct SocketMessage {
    pub payload: String,
}

impl SocketMessage {
    pub fn new(payload: &str) -> Self {
        Self {
            payload: payload.trim().to_string(),
        }
    }
}
