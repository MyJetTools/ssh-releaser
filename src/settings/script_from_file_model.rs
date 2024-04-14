use std::collections::HashMap;

use serde::*;

use crate::file_path::FilePath;

use super::RemoteCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptFromFileModel {
    pub vars: Option<HashMap<String, String>>,
    pub script: Vec<RemoteCommand>,
    #[serde(skip)]
    pub current_path: Option<FilePath>,
}
