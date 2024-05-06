use std::collections::HashMap;

use serde::*;

use super::StepModel;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSettingsModel {
    pub vars: HashMap<String, String>,
    pub var_files: Option<Vec<String>>,
    pub steps: Vec<StepModel>,
}
