use serde::*;

use crate::{file_name::FileName, script_environment::ScriptEnvironment};

use super::SshConfig;

use std::path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeSettingsModel {
    pub working_dir: String,
    pub vars: std::collections::HashMap<String, String>,
    pub ssh: Vec<SshConfig>,
}

impl HomeSettingsModel {
    pub fn get_file_name(
        &self,
        script_env: Option<&impl ScriptEnvironment>,
        file_name: &str,
    ) -> FileName {
        let mut result = if file_name.starts_with(".") {
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
