use std::time::Duration;

use crate::{
    app::AppContext,
    settings::{RemoteCommandItem, ScriptModel},
};

const EXECUTE_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn execute_commands(
    app: &AppContext,
    script_model: &ScriptModel,
    ssh: &str,
    commands: &[RemoteCommandItem],
) {
    let ssh_session = app.get_ssh_session(ssh).await;
    for command in commands {
        let command_name =
            crate::scripts::populate_variables(app, Some(script_model), command.name.as_str())
                .await;

        println!("Executing SSH command: {}", command_name.as_str());

        let command_to_exec = get_command(app, script_model, command).await;
        println!(">> {}", command_to_exec);

        let (result, status_code) = ssh_session
            .execute_command(&command_to_exec, EXECUTE_TIMEOUT)
            .await
            .unwrap();

        println!("[{}]. Result: {}", status_code, result);

        if !command.ignore_error {
            if status_code != 0 {
                panic!("Command failed with status code {}", status_code);
            }
        }
    }
}

async fn get_command(
    app: &AppContext,
    script_model: &ScriptModel,
    command: &RemoteCommandItem,
) -> String {
    if let Some(command_to_execute) = &command.exec {
        let command_to_execute =
            crate::scripts::populate_variables(app, Some(script_model), command_to_execute).await;
        return command_to_execute.to_string();
    }

    if let Some(exec_from_file) = &command.exec_from_file {
        let content = crate::scripts::load_file_and_populate_placeholders(
            app,
            Some(script_model),
            exec_from_file,
        )
        .await;
        return content;
    }

    panic!(
        "Command {} must have either 'exec' or 'exec_from_file' property",
        command.name
    )
}
