use std::collections::HashMap;

use serde::*;

use crate::settings::ExternalVariablesModel;

use super::{SettingsModel, SshConfig, StepModel};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSettingsModel {
    pub vars: HashMap<String, String>,
    pub var_files: Option<Vec<String>>,
    pub ssh: Vec<SshConfig>,
    pub steps: Vec<StepModel>,
    pub execute_step: String,
}

impl ReleaseSettingsModel {
    pub async fn load(settings: &SettingsModel) -> Self {
        let release_settings = settings.get_file_name("release.yaml");

        let content = tokio::fs::read(release_settings.clone()).await.unwrap();

        println!("Loading release settings from: {}", release_settings);

        let mut release_settings: Self = serde_yaml::from_slice(content.as_slice()).unwrap();

        if let Some(var_files) = release_settings.var_files.clone() {
            for var_file in var_files {
                let file_name = settings.get_file_name(var_file.as_str());

                let content = tokio::fs::read(file_name.as_str()).await.unwrap();
                let external_vars: ExternalVariablesModel =
                    serde_yaml::from_slice(content.as_slice()).unwrap();

                for (key, value) in external_vars.vars {
                    if release_settings.vars.contains_key(key.as_str()) {
                        panic!("Variable {} already defined", key);
                    }

                    release_settings.vars.insert(key, value);
                }
            }
        }

        release_settings
    }
}
