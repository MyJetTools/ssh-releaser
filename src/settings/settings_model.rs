use std::path;

use serde::*;

use crate::app::AppContext;

use super::RemoteCommand;

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    working_dir: String,
}

impl SettingsModel {
    pub fn get_file_name(&self, file_name: &str) -> String {
        let mut result = self
            .working_dir
            .replace("~", std::env::var("HOME").unwrap().as_str());

        if !result.ends_with(path::MAIN_SEPARATOR) {
            result.push(path::MAIN_SEPARATOR);
        }

        if file_name.starts_with(path::MAIN_SEPARATOR) {
            result.push_str(&file_name[1..]);
        } else {
            result.push_str(file_name);
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepModel {
    pub id: String,
    pub script: Option<Vec<RemoteCommand>>,
    pub from_file: Option<String>,
}

impl StepModel {
    pub async fn get_remote_commands(&self, app: &AppContext) -> Vec<RemoteCommand> {
        if let Some(script) = self.script.as_ref() {
            return script.clone();
        }

        if let Some(from_file) = self.from_file.as_ref() {
            let file_content = crate::scripts::load_file(app, from_file).await;

            let result: ScriptFromFileModel = serde_yaml::from_str(&file_content).unwrap();
            return result.script;
        }

        panic!("Please specify either script or from_file in the step model")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptFromFileModel {
    pub script: Vec<RemoteCommand>,
}
