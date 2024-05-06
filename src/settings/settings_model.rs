use std::path;

use crate::{file_name::FileName, script_environment::ScriptEnvironment};

use super::{GlobalSettingsModel, HomeSettingsModel};

pub struct SettingsModel {
    pub global_settings: GlobalSettingsModel,
    pub home_settings: HomeSettingsModel,
}

impl SettingsModel {
    pub fn get_file_name(
        &self,
        script_env: Option<&impl ScriptEnvironment>,
        file_name: &str,
    ) -> FileName {
        let mut result = if file_name.starts_with("~") {
            self.global_settings.home_dir.to_string()
        } else if file_name.starts_with(".") {
            if let Some(script_env) = script_env {
                let current_path = script_env.get_current_path().unwrap();
                current_path.as_str().to_string()
            } else {
                self.home_settings.working_dir.to_string()
            }
        } else {
            self.home_settings.working_dir.to_string()
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
}
