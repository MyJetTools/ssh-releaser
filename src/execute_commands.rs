use std::time::Duration;

use crate::{app::AppContext, settings::RemoteCommandItem};

const EXECUTE_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn execute_commands(app: &AppContext, ssh: &str, commands: &[RemoteCommandItem]) {
    let ssh_session = app.get_ssh_session(ssh).await;
    for command in commands {
        println!("Executing SSH command: {}", command.name);

        let command_to_exec = get_command(app, command).await;
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

async fn get_command(app: &AppContext, command: &RemoteCommandItem) -> String {
    if command.exec.is_some() {
        return command.exec.as_ref().unwrap().clone();
    }

    if command.exec_from_file.is_some() {
        let file_name = app
            .settings
            .get_file_name(command.exec_from_file.as_ref().unwrap());
        let content = tokio::fs::read_to_string(file_name).await.unwrap();
        return content;
    }

    panic!(
        "Command {} must have either 'exec' or 'exec_from_file' property",
        command.name
    )
}
