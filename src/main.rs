use settings::SettingsModel;

mod app;
mod execute_commands;
mod execute_post_request;
mod http_over_ssh;
mod scripts;
mod settings;
mod upload_file;

#[tokio::main]
async fn main() {
    let my_settings = SettingsModel::read_from_file(".release_config".to_string())
        .await
        .unwrap();
    let app = app::AppContext::new(my_settings.clone()).await;

    for step in &app.release_settings.steps {
        let can_execute = app.release_settings.execute_step == "*"
            || step.id == app.release_settings.execute_step;

        if !can_execute {
            continue;
        }

        println!("Executing step: {}", step.id);

        for remote_command in &step.script {
            println!("-----------------");
            if let Some(name) = remote_command.name.as_ref() {
                println!("Executing Script step: {}", name);
            }

            match remote_command.get_remote_command_type() {
                settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                    execute_commands::execute_commands(&app, &ssh, commands.as_slice()).await;
                }

                settings::RemoteCommandType::UploadFile { ssh, file } => {
                    upload_file::upload_file(&app, &ssh, file).await;
                }

                settings::RemoteCommandType::PostRequest { ssh, data } => {
                    execute_post_request::execute_post_request(&app, &ssh, &data).await;
                }
            }
        }
    }
}
