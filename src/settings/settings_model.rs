use std::path;

use serde::*;

use super::{HomeSettingsModel, RemoteCommand};

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    //working_dir: String,
    home_dir: String, // Script step would use this director as home directory which is going to be resolved by ~ symbol
}

impl SettingsModel {
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
     */

    pub fn post_process(&mut self) {
        if self.home_dir.starts_with("~") {
            self.home_dir = self
                .home_dir
                .replace("~", std::env::var("HOME").unwrap().as_str());
        }
    }

    pub async fn load_home_settings(&self) -> HomeSettingsModel {
        let mut file_name = self.home_dir.clone();
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepModel {
    pub id: String,
    pub script: Option<Vec<RemoteCommand>>,
    pub labels: Option<Vec<String>>,
    pub from_file: Option<String>,
}
