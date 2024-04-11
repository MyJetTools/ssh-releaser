use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalVarsModel {
    pub vars: std::collections::HashMap<String, String>,
}
