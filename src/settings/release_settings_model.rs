use std::collections::HashMap;

use serde::*;

use super::{SshConfig, StepModel};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSettingsModel {
    pub vars: HashMap<String, String>,
    pub ssh: Vec<SshConfig>,
    pub steps: Vec<StepModel>,
    pub execute_step: String,
}
