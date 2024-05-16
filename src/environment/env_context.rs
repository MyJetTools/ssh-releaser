use std::{collections::HashMap, path, sync::Arc};

use my_ssh::{SshCredentials, SshSession};
use rust_extensions::StrOrString;
use tokio::sync::Mutex;

use crate::{
    execution::*,
    file_name::FileName,
    settings::{
        CloudFlareConfig, HomeSettingsModel, ReleaseSettingsModel, ScriptModel, SshConfig,
        StepModel,
    },
};

use super::EnvVariables;

pub struct EnvContext {
    pub home_dir: String,
    pub working_dir: String,
    env_variables: EnvVariables,
    //home_settings: HomeSettingsModel,
    //release_settings: ReleaseSettingsModel,
    ssh_sessions: Mutex<HashMap<String, Arc<SshSession>>>,

    pub ssh: Vec<SshConfig>,
    pub cloud_flare: Option<Vec<CloudFlareConfig>>,
    steps: Vec<StepModel>,
}

impl EnvContext {
    pub async fn new(
        home_dir: String,
        home_settings: HomeSettingsModel,
        logs: &Arc<ExecuteLogsContainer>,
    ) -> Result<Self, ExecuteCommandError> {
        // let release_settings = settings.get_file_name(script_env, "release.yaml");

        let file_name = home_settings.get_release_yaml_file_name();

        let release_settings = ReleaseSettingsModel::load(file_name).await?;

        let vars_from_files = release_settings
            .load_vars_from_files(
                |file_name| {
                    let script_env: Option<&ScriptModel> = None;
                    get_file_name(&home_dir, &home_settings.working_dir, script_env, file_name)
                },
                logs,
            )
            .await?;

        let result = Self {
            home_dir,
            working_dir: home_settings.working_dir,
            env_variables: EnvVariables::new(
                home_settings.vars,
                release_settings.vars,
                vars_from_files,
            ),
            ssh: home_settings.ssh,
            //release_settings: Mutex::new(None),
            ssh_sessions: Mutex::new(HashMap::new()),
            cloud_flare: home_settings.cloud_flare,
            steps: release_settings.steps,
        };

        Ok(result)
    }

    pub fn get_file_name(
        &self,
        script_env: Option<&impl ScriptEnvironment>,
        file_name: &str,
    ) -> FileName {
        get_file_name(&self.home_dir, &self.working_dir, script_env, file_name)
    }

    pub fn get_ssh_credentials(&self, id: &str) -> Arc<SshCredentials> {
        let ssh_config = self.ssh.iter().find(|ssh| ssh.id == id);

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

        self.env_variables.get(name)
    }

    pub fn find_cloud_flare_config(&self, domain_to_write: &str) -> Option<&CloudFlareConfig> {
        if let Some(configs) = self.cloud_flare.as_ref() {
            for cloud_flare_config in configs {
                if domain_to_write.ends_with(cloud_flare_config.id.as_str()) {
                    return Some(cloud_flare_config);
                }
            }
        }

        None
    }

    pub async fn execute_me(
        &self,
        logs: &Arc<ExecuteLogsContainer>,
        step: &StepModel,
        arg: &str,
        selected_feature: Option<&str>,
    ) -> bool {
        for execute_step in arg.split(';') {
            if let Some(selected_feature) = selected_feature.as_ref() {
                if let Some(features_include) = step.features_include.as_ref() {
                    if !features_include.iter().any(|itm| itm == selected_feature) {
                        logs.write_warning(format!(
                            "Step {} is not included in the feature {}. Skipping",
                            step.id, selected_feature
                        ))
                        .await;
                        return false;
                    }
                }

                if let Some(features_exclude) = step.features_exclude.as_ref() {
                    for feature_exclude in features_exclude {
                        if feature_exclude == selected_feature {
                            logs.write_warning(format!(
                                "Step {} is excluded in the feature {}. Skipping",
                                step.id, selected_feature
                            ))
                            .await;
                            return false;
                        }
                    }
                }
            }

            if execute_step == "*" {
                return true;
            }

            if execute_step == &step.id {
                return true;
            }

            if let Some(labels) = step.labels.as_ref() {
                for label in labels {
                    if label == execute_step {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_execution_steps(&self) -> &[StepModel] {
        &self.steps
    }
}

fn get_file_name(
    home_dir: &str,
    working_dir: &str,
    script_env: Option<&impl ScriptEnvironment>,
    file_name: &str,
) -> FileName {
    if file_name.starts_with("http") {
        return FileName::new(file_name.to_string());
    }

    let mut result = if file_name.starts_with("~") {
        home_dir.to_string()
    } else if file_name.starts_with(".") {
        if let Some(script_env) = script_env {
            let current_path = script_env.get_current_path().unwrap();
            current_path.as_str().to_string()
        } else {
            working_dir.to_string()
        }
    } else {
        working_dir.to_string()
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
