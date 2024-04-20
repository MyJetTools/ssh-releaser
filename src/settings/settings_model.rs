use std::path;

use serde::*;

use crate::{file_name::FileName, script_environment::ScriptEnvironment};

use super::{GlobalSettingsModel, RemoteCommand};

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    working_dir: String,
    home_dir: String, // Script step would use this director as home directory which is going to be resolved by ~ symbol
}

impl SettingsModel {
    pub fn get_file_name(
        &self,
        script_env: Option<&impl ScriptEnvironment>,
        file_name: &str,
    ) -> FileName {
        let mut result = if file_name.starts_with("~") {
            self.home_dir.to_string()
        } else if file_name.starts_with(".") {
            if let Some(script_env) = script_env {
                let current_path = script_env.get_current_path().unwrap();
                current_path.as_str().to_string()
            } else {
                self.working_dir.to_string()
            }
        } else {
            self.working_dir.to_string()
        };

        if !result.ends_with(path::MAIN_SEPARATOR) {
            result.push(path::MAIN_SEPARATOR);
        }

        if file_name.starts_with(path::MAIN_SEPARATOR) {
            result.push_str(&file_name[1..]);
        } else if file_name.starts_with("~/") || file_name.starts_with("./") {
            result.push_str(&file_name[2..]);
        } else {
            result.push_str(&file_name);
        }

        FileName::new(result)
    }

    pub async fn read_global_settings(&self) -> GlobalSettingsModel {
        let file_name = self.get_global_settings_file_name();
        println!("Reading global vars from file: {}", file_name.as_str());
        let content = tokio::fs::read(file_name.as_str()).await;

        if let Err(err) = &content {
            panic!(
                "Can not read global vars from file {}. Err: {}",
                file_name.as_str(),
                err
            )
        }

        serde_yaml::from_slice(content.as_ref().unwrap()).unwrap()
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

        /*
        if self.global_vars.starts_with("~") {
            self.global_vars = self
                .global_vars
                .replace("~", std::env::var("HOME").unwrap().as_str());
        }
         */
    }

    fn get_global_settings_file_name(&self) -> FileName {
        let mut result = self.home_dir.to_string();

        if !result.ends_with(path::MAIN_SEPARATOR) {
            result.push(path::MAIN_SEPARATOR);
        }

        result.push_str("settings.yaml");

        FileName::new(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepModel {
    pub id: String,
    pub script: Option<Vec<RemoteCommand>>,
    pub from_file: Option<String>,
}
