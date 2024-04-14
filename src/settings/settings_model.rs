use std::{collections::HashMap, path};

use serde::*;

use crate::{app::AppContext, file_name::FileName, script_environment::ScriptEnvironment};

use super::{GlobalVarsModel, RemoteCommand};

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    working_dir: String,
    home_dir: String, // Script step would use this director as home directory which is going to be resolved by ~ symbol
    global_vars: String, // Application is going to apply global variables from this file
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
                current_path.to_string()
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

    pub async fn read_global_vars(&self) -> GlobalVarsModel {
        println!(
            "Reading global vars from file: {}",
            self.global_vars.as_str()
        );
        let content = tokio::fs::read(self.global_vars.as_str()).await;

        if let Err(err) = &content {
            panic!(
                "Can not read global vars from file {}. Err: {}",
                self.global_vars.as_str(),
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
    pub async fn get_script(&self, app: &AppContext) -> ScriptModel {
        if let Some(script) = self.script.as_ref() {
            return ScriptModel {
                script: script.clone(),
                vars: None,
                current_path: None,
            };
        }

        if let Some(from_file) = self.from_file.as_ref() {
            let script_env: Option<&ScriptModel> = None;
            let (file_content, file_name) =
                crate::scripts::load_file(app, script_env, from_file).await;

            let mut result: ScriptModel = serde_yaml::from_str(&file_content).unwrap();

            result.current_path = Some(file_name.get_file_path().to_owned());
            return result;
        }

        panic!("Please specify either script or from_file in the step model")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptModel {
    pub vars: Option<HashMap<String, String>>,
    pub script: Vec<RemoteCommand>,
    #[serde(skip)]
    current_path: Option<String>,
}

impl ScriptEnvironment for ScriptModel {
    fn get_var(&self, key: &str) -> Option<&str> {
        if let Some(vars) = self.vars.as_ref() {
            return vars.get(key).map(|itm| itm.as_str());
        }

        None
    }

    fn get_current_path(&self) -> Option<&str> {
        let result = self.current_path.as_ref()?;
        Some(result.as_str())
    }
}
