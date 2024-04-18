use std::collections::HashMap;

use serde::*;

use crate::settings::{ExternalVariablesModel, ScriptModel};

use super::{SettingsModel, SshConfig, StepModel};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSettingsModel {
    pub vars: HashMap<String, String>,
    pub var_files: Option<Vec<String>>,
    pub steps: Vec<StepModel>,
    pub execute_steps: Vec<String>,
}

impl ReleaseSettingsModel {
    pub fn execute_me(&self, id: &str) -> bool {
        for execute_step in self.execute_steps.iter() {
            if execute_step == "*" {
                return true;
            }

            if execute_step == id {
                return true;
            }
        }

        false
    }

    pub async fn load(settings: &SettingsModel) -> (Self, Vec<SshConfig>) {
        let script_env: Option<&ScriptModel> = None;
        let release_settings = settings.get_file_name(script_env, "release.yaml");

        let content = tokio::fs::read(release_settings.as_str()).await.unwrap();

        println!(
            "Loading release settings from: {}",
            release_settings.as_str()
        );

        let mut release_settings: Self = serde_yaml::from_slice(content.as_slice()).unwrap();

        if let Some(var_files) = release_settings.var_files.clone() {
            for var_file in var_files {
                let file_name = settings.get_file_name(script_env, var_file.as_str());

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

        let global_vars = settings.read_global_vars().await;

        for (key, value) in global_vars.vars {
            if release_settings.vars.contains_key(key.as_str()) {
                panic!("Variable {} already defined", key);
            }

            release_settings.vars.insert(key, value);
        }

        (release_settings, global_vars.ssh)
    }
}
