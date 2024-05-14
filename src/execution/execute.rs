use std::sync::Arc;

use crate::{
    app::AppContext,
    execution::*,
    settings::{self, ScriptModel},
};

pub async fn execute(
    app: Arc<AppContext>,
    env: String,
    args: String,
    logs: Arc<ExecuteLogsContainer>,
) {
    let env_ctx = match app.global_settings.get_env_settings(&env, &logs).await {
        Ok(env_ctx) => env_ctx,
        Err(err) => {
            logs.write_error(err).await;
            return;
        }
    };

    for step in env_ctx.get_execution_steps() {
        if !env_ctx.execute_me(step, &args) {
            continue;
        }

        logs.start_process(step.id.to_string()).await;

        let script_model = match ScriptModel::from_step(step, &env_ctx, &logs).await {
            Ok(script_model) => script_model,
            Err(err) => {
                logs.write_error(err).await;
                return;
            }
        };

        for remote_command in script_model.get_commands() {
            if let Some(name) = remote_command.name.as_ref() {
                logs.write_log(format!("Executing Script step: {}", name))
                    .await;
            }

            match remote_command.get_remote_command_type(script_model.get_current_path()) {
                settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                    if let Err(err) =
                        execute_commands(&env_ctx, &script_model, &ssh, commands.as_slice(), &logs)
                            .await
                    {
                        logs.write_error(err).await;
                        return;
                    }
                }

                settings::RemoteCommandType::UploadFile { ssh, params, file } => {
                    if let Err(err) =
                        upload_file(&env_ctx, &script_model, params, &ssh, file, &logs).await
                    {
                        logs.write_error(err).await;
                        return;
                    }
                }

                settings::RemoteCommandType::PostRequest { ssh, data } => {
                    if let Err(err) =
                        execute_post_request(&env_ctx, &script_model, &ssh, &data, &logs).await
                    {
                        logs.write_error(err).await;
                        return;
                    }
                }

                settings::RemoteCommandType::GetRequest { ssh, data } => {
                    if let Err(err) =
                        execute_get_request(&env_ctx, &script_model, &ssh, &data, &logs).await
                    {
                        logs.write_error(err).await;
                        return;
                    }
                }
                settings::RemoteCommandType::FromTemplate {
                    from_file,
                    params,
                    script_file_path,
                } => {
                    match crate::execute_from_template::execute_from_template(
                        &env_ctx,
                        from_file,
                        script_file_path,
                        params,
                        &logs,
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(err) => {
                            logs.write_error(err).await;
                            return;
                        }
                    }
                }

                settings::RemoteCommandType::WriteCloudFlareDomainARecord(model) => {
                    match super::execute_cloud_flare_write_domain(&env_ctx, model, &logs).await {
                        Ok(_) => {}
                        Err(err) => {
                            logs.write_error(err).await;
                            return;
                        }
                    }
                }
            }
        }
    }
}