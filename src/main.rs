use settings::GlobalSettingsModel;

use crate::{script_environment::ScriptEnvironment, settings::ScriptModel};

mod app;
mod execute_cloud_flare_write_domain;
mod execute_commands;
mod execute_from_template;
mod execute_get_request;
mod execute_post_request;
mod file_name;
mod file_path;
mod http_over_ssh;
mod script_environment;
mod scripts;
mod settings;
mod upload_file;

#[tokio::main]
async fn main() {
    let arg = std::env::args().last().unwrap();

    println!("Executing with argument: {}", arg);

    let mut my_settings = GlobalSettingsModel::read_from_file(".ssh-releaser".to_string())
        .await
        .unwrap();

    my_settings.post_process();

    let app = app::AppContext::new(my_settings).await;

    for step in &app.release_settings.steps {
        if !app.settings.execute_me(step, arg.as_str()) {
            continue;
        }

        println!("Executing step: {}", step.id);

        let script_model = ScriptModel::from_step(step, &app).await;

        for remote_command in script_model.get_commands() {
            println!("-----------------");
            if let Some(name) = remote_command.name.as_ref() {
                println!("Executing Script step: {}", name);
            }

            match remote_command.get_remote_command_type(script_model.get_current_path()) {
                settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                    execute_commands::execute_commands(
                        &app,
                        &script_model,
                        &ssh,
                        commands.as_slice(),
                    )
                    .await;
                }

                settings::RemoteCommandType::UploadFile { ssh, params, file } => {
                    upload_file::upload_file(&app, &script_model, params, &ssh, file).await;
                }

                settings::RemoteCommandType::PostRequest { ssh, data } => {
                    execute_post_request::execute_post_request(&app, &script_model, &ssh, &data)
                        .await;
                }

                settings::RemoteCommandType::GetRequest { ssh, data } => {
                    execute_get_request::execute_get_request(&app, &script_model, &ssh, &data)
                        .await;
                }
                settings::RemoteCommandType::FromTemplate {
                    from_file,
                    params,
                    script_file_path,
                } => {
                    execute_from_template::execute_from_template(
                        &app,
                        from_file,
                        script_file_path,
                        params,
                    )
                    .await;
                }

                settings::RemoteCommandType::WriteCloudFlareDomainARecord(model) => {
                    crate::execute_cloud_flare_write_domain::execute_cloud_flare_write_domain(
                        &app, model,
                    )
                    .await;
                }
            }
        }
    }
}
