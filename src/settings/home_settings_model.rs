use serde::*;

use crate::{
    file_name::FileName,
    script_environment::ScriptEnvironment,
    settings::{ExternalVariablesModel, ScriptModel},
};

use super::{ReleaseSettingsModel, SettingsModel, SshConfig};

use std::path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeSettingsModel {
    pub working_dir: String,
    pub vars: std::collections::HashMap<String, String>,
    pub ssh: Vec<SshConfig>,
}

impl HomeSettingsModel {
    fn get_file_name(
        &self,
        settings: &SettingsModel,
        script_env: Option<&impl ScriptEnvironment>,
        file_name: &str,
    ) -> FileName {
        let mut result = if file_name.starts_with("~") {
            settings.home_dir.to_string()
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

    pub async fn load_release_settings(&self, settings: &SettingsModel) -> ReleaseSettingsModel {
        let script_env: Option<&ScriptModel> = None;
        let release_settings = self.get_file_name(settings, script_env, "release.yaml");

        let content = tokio::fs::read(release_settings.as_str()).await.unwrap();

        println!(
            "Loading release settings from: {}",
            release_settings.as_str()
        );

        let mut release_settings: ReleaseSettingsModel =
            serde_yaml::from_slice(content.as_slice()).unwrap();

        if let Some(var_files) = release_settings.var_files.clone() {
            for var_file in var_files {
                let file_name = self.get_file_name(settings, script_env, var_file.as_str());

                let content = tokio::fs::read(file_name.as_str()).await.unwrap();

                let external_vars: ExternalVariablesModel =
                    match serde_yaml::from_slice(content.as_slice()) {
                        Ok(result) => result,
                        Err(err) => {
                            panic!("can not load yaml: {}. Err: {}", file_name.as_str(), err)
                        }
                    };

                for (key, value) in external_vars.vars {
                    if release_settings.vars.contains_key(key.as_str()) {
                        panic!("Variable {} already defined", key);
                    }

                    release_settings.vars.insert(key, value);
                }
            }
        }

        for (key, value) in self.vars.clone() {
            if release_settings.vars.contains_key(key.as_str()) {
                panic!("Variable {} already defined", key);
            }

            release_settings.vars.insert(key, value);
        }

        release_settings
    }

    pub fn post_process(&mut self) {
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
}
