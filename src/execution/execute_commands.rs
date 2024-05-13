use std::{sync::Arc, time::Duration};

use crate::{
    environment::EnvContext,
    settings::{RemoteCommandItem, ScriptModel},
};

use super::{ExecuteCommandError, ExecuteLogsContainer};

const EXECUTE_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn execute_commands(
    env_settings: &EnvContext,
    script_model: &ScriptModel,
    ssh: &str,
    commands: &[RemoteCommandItem],
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<(), ExecuteCommandError> {
    let ssh_session = env_settings.get_ssh_session(ssh).await;
    for command in commands {
        let command_name = crate::scripts::populate_variables(
            env_settings,
            Some(script_model),
            command.name.as_str(),
            logs,
        )
        .await?;

        logs.write_log(format!("Executing SSH command: {}", command_name.as_str()))
            .await;

        let command_to_exec = get_command(env_settings, script_model, command, logs).await?;

        logs.write_log(format!(">> {}", command_to_exec)).await;

        let (result, status_code) = ssh_session
            .execute_command(&command_to_exec, EXECUTE_TIMEOUT)
            .await?;

        logs.write_log(format!("[{}]. Result: {}", status_code, result))
            .await;

        if !command.ignore_error {
            if status_code != 0 {
                logs.write_error(format!("Command failed with status code {}", status_code))
                    .await;
            }
        }
    }

    Ok(())
}

async fn get_command(
    env_settings: &EnvContext,
    script_model: &ScriptModel,
    command: &RemoteCommandItem,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<String, ExecuteCommandError> {
    if let Some(command_to_execute) = &command.exec {
        let command_to_execute = crate::scripts::populate_variables(
            env_settings,
            Some(script_model),
            command_to_execute,
            logs,
        )
        .await?;
        return Ok(command_to_execute.to_string());
    }

    if let Some(exec_from_file) = &command.exec_from_file {
        let content = crate::scripts::load_file_and_populate_placeholders(
            env_settings,
            Some(script_model),
            exec_from_file,
            logs,
        )
        .await;
        return content;
    }

    return Err(format!(
        "Command {} must have either 'exec' or 'exec_from_file' property",
        command.name
    )
    .into());
}
