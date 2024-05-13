use std::{collections::HashMap, sync::Arc};

use crate::{
    environment::EnvContext,
    execute_from_template::ReadingFromTemplateEnvironment,
    execution::{ExecuteCommandError, ExecuteLogsContainer, ScriptEnvironment},
    file_path::FilePath,
    settings::{self, ScriptModel},
};

pub async fn execute_from_template(
    env_settings: &EnvContext,
    from_file: String,
    script_file_path: FilePath,
    mut params: Option<HashMap<String, String>>,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<(), ExecuteCommandError> {
    let script_env: Option<&ScriptModel> = None;

    if let Some(params) = params.as_mut() {
        for (key, value) in params.clone() {
            let value = crate::scripts::populate_variables(env_settings, script_env, &value).await;
            params.insert(key.clone(), value.to_string());
        }
    }

    let file_name = env_settings.get_file_name(script_env, from_file.as_str());

    let content = file_name.load_content_as_string().await;

    let loading_template_env = ReadingFromTemplateEnvironment::new(params);
    let content = crate::scripts::populate_variables_after_loading_from_file(
        env_settings,
        Some(&loading_template_env),
        content,
        "*{",
    );

    let script_model = ScriptModel::from_content(content.as_str(), script_file_path.as_ref());

    for remote_command in script_model.get_commands() {
        println!("-----------------");
        if let Some(name) = remote_command.name.as_ref() {
            println!("Executing Script step: {}", name);
        }

        match remote_command.get_remote_command_type(script_model.get_current_path()) {
            settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                crate::execution::execute_commands(
                    env_settings,
                    &script_model,
                    &ssh,
                    commands.as_slice(),
                    logs,
                )
                .await?;
            }

            settings::RemoteCommandType::UploadFile { ssh, params, file } => {
                crate::execution::upload_file(
                    env_settings,
                    &script_model,
                    params,
                    &ssh,
                    file,
                    logs,
                )
                .await?;
            }

            settings::RemoteCommandType::PostRequest { ssh, data } => {
                crate::execution::execute_post_request(
                    env_settings,
                    &script_model,
                    &ssh,
                    &data,
                    logs,
                )
                .await?;
            }

            settings::RemoteCommandType::GetRequest { ssh, data } => {
                crate::execution::execute_get_request(
                    env_settings,
                    &script_model,
                    &ssh,
                    &data,
                    logs,
                )
                .await?;
            }
            settings::RemoteCommandType::FromTemplate {
                from_file,
                params,
                script_file_path,
            } => {
                return Err(format!(
                    "Nested templates are not supported yet. Please check file: {} with params: {:?}. Script file name: {}",
                    from_file, params, script_file_path.as_str()
                ).into());
            }

            settings::RemoteCommandType::WriteCloudFlareDomainARecord(model) => {
                crate::execution::execute_cloud_flare_write_domain(env_settings, model, logs)
                    .await?;
            }
        }
    }

    Ok(())
}
