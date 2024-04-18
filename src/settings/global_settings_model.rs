use serde::*;

use super::SshConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettingsModel {
    pub vars: std::collections::HashMap<String, String>,
    pub ssh: Vec<SshConfig>,
}
