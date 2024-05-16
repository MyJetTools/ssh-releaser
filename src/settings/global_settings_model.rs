use std::{collections::BTreeMap, path, sync::Arc};

use serde::*;

use crate::{
    environment::EnvContext,
    execution::{ExecuteCommandError, ExecuteLogsContainer},
};

use super::{HomeSettingsModel, RemoteCommand};

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettingsModel {
    //working_dir: String,
    envs: BTreeMap<String, String>, // Script step would use this director as home directory which is going to be resolved by ~ symbol
}

impl GlobalSettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().map(|x| x.to_string()).collect()
    }
    /*
    pub async fn read_working_settings(&self) -> HomeSettingsModel {
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
        let mut result = HashMap::new();

        for (key, value) in &self.envs {
            result.insert(key.to_string(), value);
        }

        self.envs.clear();

        for (key, value) in result {
            self.envs.insert(key, value);
        }
    }
     */
    pub async fn get_env_settings(
        &self,
        env: &str,
        logs: &Arc<ExecuteLogsContainer>,
    ) -> Result<EnvContext, ExecuteCommandError> {
        let home_dir = match self.envs.get(env) {
            Some(home_dir) => home_dir,
            None => panic!("There is not environment {} in global settings", env),
        };

        let home_dir = if home_dir.starts_with("~") {
            home_dir.replace("~", std::env::var("HOME").unwrap().as_str())
        } else {
            home_dir.clone()
        };

        let home_settings = load_home_settings(home_dir.as_str()).await;

        EnvContext::new(home_dir, home_settings, logs).await
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
    pub from_file: Option<String>,
}
