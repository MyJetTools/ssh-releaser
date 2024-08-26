use std::{collections::BTreeMap, path, sync::Arc};

use serde::*;

use crate::{
    app::AppContext,
    environment::EnvContext,
    execution::{ExecuteCommandError, ExecuteLogsContainer},
    scripts::populate_variables,
};

use super::{HomeSettingsModel, RemoteCommand};

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettingsModel {
    //working_dir: String,
    envs: BTreeMap<String, BTreeMap<String, String>>, // Script step would use this director as home directory which is going to be resolved by ~ symbol
}

impl GlobalSettingsModel {
    pub fn get_envs(&self) -> BTreeMap<String, Vec<String>> {
        let mut result = BTreeMap::new();

        for (product_code, envs) in &self.envs {
            result.insert(
                product_code.to_string(),
                envs.keys().map(|x| x.to_string()).collect(),
            );
        }

        result
    }

    pub async fn get_env_settings(
        &self,
        app: Arc<AppContext>,
        product_code: &str,
        env: &str,
        logs: &Arc<ExecuteLogsContainer>,
    ) -> Result<EnvContext, ExecuteCommandError> {
        if let Some(envs) = self.envs.get(product_code) {
            let home_dir = match envs.get(env) {
                Some(home_dir) => home_dir,
                None => panic!("There is not environment {} in global settings", env),
            };

            let home_dir = if home_dir.starts_with("~") {
                home_dir.replace("~", std::env::var("HOME").unwrap().as_str())
            } else {
                home_dir.clone()
            };

            let home_settings = load_home_settings(home_dir.as_str()).await;

            return EnvContext::new(app.clone(), home_dir, home_settings, logs).await;
        }

        panic!(
            "There is not product code {} in global settings",
            product_code
        );
    }

    /*
    pub fn get_home_dir(&self, env: &str) -> &str {
        match self.envs.get(env) {
            Some(result) => result,
            None => {
                panic!("There is not environment {} in global settings", env);
            }
        }
    }



     */
}

async fn load_home_settings(home_dir: &str) -> HomeSettingsModel {
    let mut file_name = home_dir.to_string();
    if !file_name.ends_with(path::MAIN_SEPARATOR) {
        file_name.push(path::MAIN_SEPARATOR);
    }

    file_name.push_str("settings.yaml");

    println!("Loading home settings from file: {}", file_name.as_str());

    let content = match tokio::fs::read(file_name.as_str()).await {
        Ok(result) => result,
        Err(err) => panic!("Can not read file {}. Err: {}", file_name, err),
    };

    let mut result: HomeSettingsModel = serde_yaml::from_slice(content.as_ref()).unwrap();

    result.post_process();

    result
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepModel {
    pub id: String,
    pub script: Option<Vec<RemoteCommand>>,
    pub category: Option<String>,
    pub labels: Option<Vec<String>>,
    pub features_include: Option<Vec<String>>,
    pub features_exclude: Option<Vec<String>>,
    pub from_file: Option<String>,

    // here we set reference to param from a global config to execute command several times with different values for a same key
    pub params_to_repeat: Option<ParamsToRepeat>,
}

impl StepModel {
    pub async fn get_params_to_repeat(
        &self,
        env_ctx: &EnvContext,
        logs: &std::sync::Arc<crate::execution::ExecuteLogsContainer>,
    ) -> Result<Option<ParamsToRepeatReadModel>, ExecuteCommandError> {
        let params_to_repeat = self.params_to_repeat.as_ref();

        if params_to_repeat.is_none() {
            return Ok(None);
        }

        let params_to_repeat = params_to_repeat.unwrap();

        let values = params_to_repeat.get_values(env_ctx, logs).await?;

        Ok(Some(ParamsToRepeatReadModel {
            name: params_to_repeat.name.to_string(),
            values,
        }))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamsToRepeat {
    pub name: String,
    pub values: String,
}

impl ParamsToRepeat {
    pub async fn get_values(
        &self,
        env_ctx: &EnvContext,
        logs: &std::sync::Arc<crate::execution::ExecuteLogsContainer>,
    ) -> Result<Vec<String>, ExecuteCommandError> {
        let mut result = Vec::new();

        let script_env: Option<&crate::settings::ScriptModel> = None;

        let values = populate_variables(env_ctx, script_env, &self.values, logs).await?;

        for itm in values.as_str().split("|").map(|x| x.trim()) {
            let value = populate_variables(env_ctx, script_env, itm, logs).await?;

            result.push(value.to_string());
        }

        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct ParamsToRepeatReadModel {
    pub name: String,
    pub values: Vec<String>,
}
