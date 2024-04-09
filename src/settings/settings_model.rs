use std::path;

use serde::*;

use super::RemoteCommand;

#[derive(my_settings_reader::SettingsModel, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    working_dir: String,
}

impl SettingsModel {
    pub fn get_file_name(&self, file_name: &str) -> String {
        let mut result = self
            .working_dir
            .replace("~", std::env::var("HOME").unwrap().as_str());

        if !result.ends_with(path::MAIN_SEPARATOR) {
            result.push(path::MAIN_SEPARATOR);
        }

        if file_name.starts_with(path::MAIN_SEPARATOR) {
            result.push_str(&file_name[1..]);
        } else {
            result.push_str(file_name);
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepModel {
    pub id: String,
    pub script: Vec<RemoteCommand>,
}
