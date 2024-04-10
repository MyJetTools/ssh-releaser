use std::{collections::HashMap, sync::Arc};

use my_ssh::{SshCredentials, SshSession};
use tokio::sync::Mutex;

use crate::settings::{ReleaseSettingsModel, SettingsModel};

pub struct AppContext {
    ssh_sessions: Mutex<HashMap<String, Arc<SshSession>>>,
    pub settings: SettingsModel,
    pub release_settings: ReleaseSettingsModel,
}

impl AppContext {
    pub async fn new(settings: SettingsModel) -> AppContext {
        let release_settings = ReleaseSettingsModel::load(&settings).await;
        AppContext {
            settings,
            ssh_sessions: Mutex::new(HashMap::new()),
            release_settings,
        }
    }

    pub fn get_ssh_credentials(&self, id: &str) -> Arc<SshCredentials> {
        let ssh_config = self.release_settings.ssh.iter().find(|ssh| ssh.id == id);

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
}
