use settings::SettingsModel;

mod app;
mod execute_commands;
mod execute_get_request;
mod execute_post_request;
mod file_name;
mod http_over_ssh;
mod script_environment;
mod scripts;
mod settings;
mod upload_file;

#[tokio::main]
async fn main() {
    let mut my_settings = SettingsModel::read_from_file(".ssh-releaser".to_string())
        .await
        .unwrap();

    my_settings.post_process();

    let app = app::AppContext::new(my_settings.clone()).await;

    for step in &app.release_settings.steps {
        if !app.release_settings.execute_me(&step.id) {
            continue;
        }

        println!("Executing step: {}", step.id);

        let script = step.get_script(&app).await;

        for remote_command in &script.script {
            println!("-----------------");
            if let Some(name) = remote_command.name.as_ref() {
                println!("Executing Script step: {}", name);
            }

            match remote_command.get_remote_command_type() {
                settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                    execute_commands::execute_commands(&app, &script, &ssh, commands.as_slice())
                        .await;
                }

                settings::RemoteCommandType::UploadFile { ssh, file } => {
                    upload_file::upload_file(&app, &script, &ssh, file).await;
                }

                settings::RemoteCommandType::PostRequest { ssh, data } => {
                    execute_post_request::execute_post_request(&app, &script, &ssh, &data).await;
                }

                settings::RemoteCommandType::GetRequest { ssh, data } => {
                    execute_get_request::execute_get_request(&app, &script, &ssh, &data).await;
                }
            }
        }
    }
}
