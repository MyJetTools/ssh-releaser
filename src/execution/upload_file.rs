use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    environment::EnvContext,
    settings::{ScriptModel, UploadFileModel},
};

use super::{ExecuteCommandError, ExecuteLogsContainer};

pub async fn upload_file(
    env_settings: &EnvContext,
    script_model: &ScriptModel,
    params: Option<HashMap<String, String>>,
    ssh: &str,
    file: UploadFileModel,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<(), ExecuteCommandError> {
    let local_file = crate::scripts::populate_variables(
        env_settings,
        Some(script_model),
        &file.local_file.as_str(),
        logs,
    )
    .await?;

    let mut content = crate::scripts::load_file_and_populate_placeholders(
        env_settings,
        Some(script_model),
        local_file.as_str(),
        logs,
    )
    .await?;

    if let Some(params) = params {
        let env = UploadFileEnvironment::new(params);
        content = crate::scripts::populate_variables_after_loading_from_file(
            env_settings,
            Some(&env),
            content,
            "*{",
        )
    }

    let session = env_settings.get_ssh_session(ssh).await;

    let remote_file = crate::scripts::populate_variables(
        env_settings,
        Some(script_model),
        file.remote_file.as_str(),
        logs,
    )
    .await?;

    logs.write_log(format!(
        "Uploading file to remote path: {}",
        remote_file.as_str()
    ))
    .await;

    let result = session
        .upload_file(
            remote_file.as_str(),
            content.as_bytes(),
            file.mode,
            Duration::from_secs(30),
        )
        .await?;

    logs.write_log(format!("File uploaded with result: {}", result))
        .await;

    Ok(())
}

pub struct UploadFileEnvironment {
    params: HashMap<String, String>,
}

impl UploadFileEnvironment {
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }
}

impl crate::execution::ScriptEnvironment for UploadFileEnvironment {
    fn get_var(&self, name: &str) -> Option<&str> {
        let result = self.params.get(name)?;
        Some(result)
    }

    fn get_current_path<'s>(&'s self) -> Option<crate::file_path::FilePathRef<'s>> {
        None
    }
}
