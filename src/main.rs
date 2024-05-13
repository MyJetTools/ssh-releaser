use std::sync::Arc;

use settings::GlobalSettingsModel;

mod app;

mod environment;

mod execute_from_template;

mod execution;
mod file_name;
mod file_path;
mod http;
mod http_over_ssh;

mod scripts;
mod settings;

#[tokio::main]
async fn main() {
    //    let arg = std::env::args().last().unwrap();

    //    println!("Executing with argument: {}", arg);
    //let env_param = AppEnvParams::new(arg);

    let my_settings = GlobalSettingsModel::read_from_file(".ssh-releaser".to_string())
        .await
        .unwrap();

    // my_settings.post_process();

    let app = Arc::new(app::AppContext::new(my_settings).await);

    crate::http::setup_server(&app);

    app.app_states.wait_until_shutdown().await;

    /*

    let env_ctx = app
        .global_settings
        .get_env_settings(env_param.get_env())
        .await;

    for step in env_ctx.get_execution_steps() {
        if !env_ctx.execute_me(step, env_param.get_label()) {
            continue;
        }

        println!("Executing step: {}", step.id);

        let script_model = ScriptModel::from_step(step, &env_ctx).await;

        for remote_command in script_model.get_commands() {
            println!("-----------------");
            if let Some(name) = remote_command.name.as_ref() {
                println!("Executing Script step: {}", name);
            }

            match remote_command.get_remote_command_type(script_model.get_current_path()) {
                settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                    execute_commands::execute_commands(
                        &env_ctx,
                        &script_model,
                        &ssh,
                        commands.as_slice(),
                    )
                    .await;
                }

                settings::RemoteCommandType::UploadFile { ssh, params, file } => {
                    upload_file::upload_file(&env_ctx, &script_model, params, &ssh, file).await;
                }

                settings::RemoteCommandType::PostRequest { ssh, data } => {
                    execute_post_request::execute_post_request(
                        &env_ctx,
                        &script_model,
                        &ssh,
                        &data,
                    )
                    .await;
                }

                settings::RemoteCommandType::GetRequest { ssh, data } => {
                    execute_get_request::execute_get_request(&env_ctx, &script_model, &ssh, &data)
                        .await;
                }
                settings::RemoteCommandType::FromTemplate {
                    from_file,
                    params,
                    script_file_path,
                } => {
                    execute_from_template::execute_from_template(
                        &env_ctx,
                        from_file,
                        script_file_path,
                        params,
                    )
                    .await;
                }

                settings::RemoteCommandType::WriteCloudFlareDomainARecord(model) => {
                    crate::execute_cloud_flare_write_domain::execute_cloud_flare_write_domain(
                        &env_ctx, model,
                    )
                    .await;
                }
            }
        }
    }
     */
}
