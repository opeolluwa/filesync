use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddedServerConfig {
    pub ip_address: String,
}
