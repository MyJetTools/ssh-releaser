use std::{collections::HashMap, sync::Arc};

use my_ssh::{SshCredentials, SshSession};
use rust_extensions::StrOrString;
use tokio::sync::Mutex;

use crate::{file_name::FileName, script_environment::ScriptEnvironment};

use std::path;

use crate::settings::{HomeSettingsModel, ReleaseSettingsModel, SettingsModel};

pub struct AppContext {
    ssh_sessions: Mutex<HashMap<String, Arc<SshSession>>>,
    pub settings: SettingsModel,
    pub home_settings: HomeSettingsModel,
    pub release_settings: ReleaseSettingsModel,
}

impl AppContext {
    pub async fn new(settings: SettingsModel) -> AppContext {
        let home_settings = settings.load_home_settings().await;
        let release_settings = home_settings.load_release_settings(&settings).await;
        AppContext {
            settings,
            ssh_sessions: Mutex::new(HashMap::new()),
            release_settings,
            home_settings,
        }
    }

    pub fn get_ssh_credentials(&self, id: &str) -> Arc<SshCredentials> {
        let ssh_config = self.home_settings.ssh.iter().find(|ssh| ssh.id == id);

        if ssh_config.is_none() {
            panic!("SSH config with id {} not found", id);
        }

        let ssh_config = ssh_config.unwrap();

        Arc::new(SshCredentials::SshAgent {
            ssh_remote_host: ssh_config.host.clone(),
            ssh_remote_port: ssh_config.port,
            ssh_user_name: ssh_config.user_name.clone(),
        })
    }

    pub async fn get_ssh_session(&self, id: &str) -> Arc<SshSession> {
        let mut ssh_sessions = self.ssh_sessions.lock().await;

        if ssh_sessions.contains_key(id) {
            return ssh_sessions.get(id).unwrap().clone();
        }

        let ssh_credentials = self.get_ssh_credentials(id);

        let session = Arc::new(SshSession::new(ssh_credentials));

        ssh_sessions.insert(id.to_string(), session.clone());

        session
    }

    pub fn get_env_variable<'s>(
        &'s self,
        script_env: Option<&'s impl ScriptEnvironment>,
        name: &str,
    ) -> StrOrString<'s> {
        if let Some(script_env) = script_env {
            if let Some(value) = script_env.get_var(name) {
                return value.into();
            }
        }

        if let Some(value) = self.release_settings.vars.get(name) {
            return value.into();
        }

        if let Ok(value) = std::env::var(name) {
            return value.into();
        }

        panic!("Variable {} not found", name);
    }

    pub fn get_file_name(
        &self,
        script_env: Option<&impl ScriptEnvironment>,
        file_name: &str,
    ) -> FileName {
        let mut result = if file_name.starts_with("~") {
            self.settings.home_dir.to_string()
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
