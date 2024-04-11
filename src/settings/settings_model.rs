use std::path;

use serde::*;

use crate::app::AppContext;

use super::{GlobalVarsModel, RemoteCommand};

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    working_dir: String,
    home_dir: String, // Script step would use this director as home directory which is going to be resolved by ~ symbol
    global_vars: String, // Application is going to apply global variables from this file
}

impl SettingsModel {
    pub fn get_file_name(&self, file_name: &str) -> String {
        let mut result = if file_name.starts_with("~") {
            self.home_dir.to_string()
        } else {
            self.working_dir.to_string()
        };

        if !result.ends_with(path::MAIN_SEPARATOR) {
            result.push(path::MAIN_SEPARATOR);
        }

        if file_name.starts_with(path::MAIN_SEPARATOR) {
            result.push_str(&file_name[1..]);
        } else if file_name.starts_with("~/") {
            result.push_str(&file_name[2..]);
        } else {
            result.push_str(&file_name);
        }

        result
    }

    pub async fn read_global_vars(&self) -> GlobalVarsModel {
        let file_name = self.get_file_name(self.global_vars.as_str());

        let content = tokio::fs::read(file_name.clone()).await.unwrap();

        serde_yaml::from_slice(content.as_slice()).unwrap()
    }

    pub fn post_process(&mut self) {
        if self.home_dir.starts_with("~") {
            self.home_dir = self
                .home_dir
                .replace("~", std::env::var("HOME").unwrap().as_str());
        }

        if self.working_dir.starts_with("~") {
            self.working_dir = self
                .working_dir
                .replace("~", std::env::var("HOME").unwrap().as_str());
        }

        if self.global_vars.starts_with("~") {
            self.global_vars = self
                .global_vars
                .replace("~", std::env::var("HOME").unwrap().as_str());
        }
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
