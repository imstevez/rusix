use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configs {
    pub host: String,
    pub port: u16,
}
