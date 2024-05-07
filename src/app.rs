use std::{collections::HashMap, sync::Arc};

use my_ssh::{SshCredentials, SshSession};
use rust_extensions::StrOrString;
use tokio::sync::Mutex;

use crate::{script_environment::ScriptEnvironment, settings::SettingsModel};

use crate::settings::{CloudFlareConfig, GlobalSettingsModel, ReleaseSettingsModel};

pub struct AppContext {
    ssh_sessions: Mutex<HashMap<String, Arc<SshSession>>>,
    pub settings: SettingsModel,
    pub release_settings: ReleaseSettingsModel,
}

impl AppContext {
    pub async fn new(global_settings: GlobalSettingsModel) -> AppContext {
        let home_settings = global_settings.load_home_settings().await;

        let settings = SettingsModel {
            global_settings,
            home_settings: home_settings,
        };
        let release_settings = settings
            .home_settings
            .load_release_settings(&settings)
            .await;

        AppContext {
            settings,
            ssh_sessions: Mutex::new(HashMap::new()),
            release_settings,
        }
    }

    pub fn get_ssh_credentials(&self, id: &str) -> Arc<SshCredentials> {
        let ssh_config = self
            .settings
            .home_settings
            .ssh
            .iter()
            .find(|ssh| ssh.id == id);

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

    pub fn find_cloud_flare_config(&self, domain_to_write: &str) -> Option<&CloudFlareConfig> {
        if let Some(configs) = self.settings.home_settings.cloud_flare.as_ref() {
            for cloud_flare_config in configs {
                if domain_to_write.ends_with(cloud_flare_config.id.as_str()) {
                    return Some(cloud_flare_config);
                }
            }
        }

        None
    }
}
