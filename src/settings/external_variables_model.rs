use serde::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalVariablesModel {
    pub vars: HashMap<String, String>,
}
