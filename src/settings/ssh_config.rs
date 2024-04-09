use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub user_name: String,
}
