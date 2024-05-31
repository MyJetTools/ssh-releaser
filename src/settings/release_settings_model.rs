use std::{collections::HashMap, sync::Arc};

use serde::*;

use crate::{
    app::AppContext,
    execution::{ExecuteCommandError, ExecuteLogsContainer},
    file_name::FileName,
};

use super::{ExternalVariablesModel, StepModel};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSettingsModel {
    pub vars: HashMap<String, String>,
    pub var_files: Option<Vec<String>>,
    pub steps: Vec<StepModel>,
}

impl ReleaseSettingsModel {
    pub async fn load(release_file_name: FileName) -> Result<Self, ExecuteCommandError> {
        let content = tokio::fs::read(release_file_name.as_str()).await?;
        let release_settings: Result<Self, _> = serde_yaml::from_slice(content.as_slice());

        match release_settings {
            Ok(result) => Ok(result),
            Err(err) => Err(ExecuteCommandError::JustError(format!(
                "can not load yaml: {}. Err: {}",
                release_file_name.as_str(),
                err
            ))),
        }

        /*
        let script_env: Option<&ScriptModel> = None;

        let content = tokio::fs::read(release_file_name.as_str()).await.unwrap();

        println!(
            "Loading release settings from: {}",
            release_file_name.as_str()
        );

        let mut release_settings: ReleaseSettingsModel =
            serde_yaml::from_slice(content.as_slice()).unwrap();

        if let Some(var_files) = release_settings.var_files.clone() {
            for var_file in var_files {
                let file_name = settings.get_file_name(script_env, var_file.as_str());

                let content = file_name.load_content().await;

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

        for (key, value) in home_settings_var {
            if release_settings.vars.contains_key(key.as_str()) {
                panic!("Variable {} already defined", key);
            }

            release_settings
                .vars
                .insert(key.to_string(), value.to_string());
        }

        release_settings

         */
    }

    pub async fn load_vars_from_files(
        &self,
        app: &AppContext,
        get_file_name: impl Fn(&str) -> FileName,
        logs: &Arc<ExecuteLogsContainer>,
    ) -> Result<HashMap<String, ExternalVariablesModel>, ExecuteCommandError> {
        //  let script_env: Option<&ScriptModel> = None;

        let mut result = HashMap::new();
        if let Some(var_files) = &self.var_files {
            for var_file in var_files {
                //   let file_name = settings.get_file_name(script_env, var_file.as_str());

                let file_name = get_file_name(var_file);

                let content = file_name.load_content(app, logs).await?;

                let external_vars: ExternalVariablesModel =
                    match serde_yaml::from_slice(content.as_slice()) {
                        Ok(result) => result,
                        Err(err) => {
                            panic!("can not load yaml: {}. Err: {}", file_name.as_str(), err)
                        }
                    };

                result.insert(file_name.as_str().to_string(), external_vars);
            }
        }

        Ok(result)
    }
}
