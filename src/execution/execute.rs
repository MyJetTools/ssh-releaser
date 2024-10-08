use std::sync::Arc;

use crate::{
    app::AppContext,
    execution::*,
    settings::{self, ScriptModel},
};

pub async fn execute(
    app: Arc<AppContext>,
    product_code: &str,
    env: String,
    args: String,
    logs: Arc<ExecuteLogsContainer>,
) -> Result<(), ExecuteCommandError> {
    let mut env_ctx = app
        .global_settings
        .get_env_settings(app.clone(), product_code, &env, &logs)
        .await?;

    for step in env_ctx.get_execution_steps() {
        if !env_ctx.execute_me(&logs, &step, &args).await {
            continue;
        }

        let params_to_repeat = step.get_params_to_repeat(&env_ctx, &logs).await?;

        logs.start_process(step.id.to_string()).await;

        let script_model = ScriptModel::from_step(&step, &env_ctx, &logs).await?;

        for remote_command in script_model.get_commands() {
            if let Some(name) = remote_command.name.as_ref() {
                logs.write_log(format!("Executing Script step: {}", name))
                    .await;
            }

            match remote_command.get_remote_command_type(script_model.get_current_path()) {
                settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                    if let Some(params_to_repeat) = &params_to_repeat {
                        for value in &params_to_repeat.values {
                            env_ctx
                                .apply_step_repeat_parameter(
                                    Some((params_to_repeat.name.as_str(), value)),
                                    &logs,
                                )
                                .await;
                            execute_commands(
                                &env_ctx,
                                &script_model,
                                &ssh,
                                commands.as_slice(),
                                &logs,
                            )
                            .await?;
                        }
                    } else {
                        env_ctx.apply_step_repeat_parameter(None, &logs).await;
                        execute_commands(&env_ctx, &script_model, &ssh, commands.as_slice(), &logs)
                            .await?;
                    }
                }

                settings::RemoteCommandType::UploadFile { ssh, params, file } => {
                    if let Some(params_to_repeat) = &params_to_repeat {
                        for value in &params_to_repeat.values {
                            env_ctx
                                .apply_step_repeat_parameter(
                                    Some((params_to_repeat.name.as_str(), value)),
                                    &logs,
                                )
                                .await;

                            upload_file(
                                &env_ctx,
                                &script_model,
                                params.clone(),
                                &ssh,
                                &file,
                                &logs,
                            )
                            .await?;
                        }
                    } else {
                        env_ctx.apply_step_repeat_parameter(None, &logs).await;
                        upload_file(&env_ctx, &script_model, params.clone(), &ssh, &file, &logs)
                            .await?;
                    }
                }

                settings::RemoteCommandType::PostRequest { ssh, data } => {
                    if let Some(params_to_repeat) = &params_to_repeat {
                        for value in &params_to_repeat.values {
                            env_ctx
                                .apply_step_repeat_parameter(
                                    Some((params_to_repeat.name.as_str(), value)),
                                    &logs,
                                )
                                .await;
                            execute_post_request(&env_ctx, &script_model, &ssh, &data, &logs)
                                .await?;
                        }
                    } else {
                        env_ctx.apply_step_repeat_parameter(None, &logs).await;
                        execute_post_request(&env_ctx, &script_model, &ssh, &data, &logs).await?;
                    }
                }

                settings::RemoteCommandType::GetRequest { ssh, data } => {
                    if let Some(params_to_repeat) = &params_to_repeat {
                        for value in &params_to_repeat.values {
                            env_ctx
                                .apply_step_repeat_parameter(
                                    Some((params_to_repeat.name.as_str(), value)),
                                    &logs,
                                )
                                .await;
                            execute_get_request(&env_ctx, &script_model, &ssh, &data, &logs)
                                .await?;
                        }
                    } else {
                        env_ctx.apply_step_repeat_parameter(None, &logs).await;
                        execute_get_request(&env_ctx, &script_model, &ssh, &data, &logs).await?;
                    }
                }
                settings::RemoteCommandType::FromTemplate {
                    from_file,
                    params,
                    script_file_path,
                } => {
                    if let Some(params_to_repeat) = &params_to_repeat {
                        for value in &params_to_repeat.values {
                            env_ctx
                                .apply_step_repeat_parameter(
                                    Some((params_to_repeat.name.as_str(), value)),
                                    &logs,
                                )
                                .await;
                            crate::execute_from_template::execute_from_template(
                                &env_ctx,
                                &from_file,
                                &script_file_path,
                                params.clone(),
                                &logs,
                            )
                            .await?;
                        }
                    } else {
                        env_ctx.apply_step_repeat_parameter(None, &logs).await;
                        crate::execute_from_template::execute_from_template(
                            &env_ctx,
                            &from_file,
                            &script_file_path,
                            params.clone(),
                            &logs,
                        )
                        .await?;
                    }
                }

                settings::RemoteCommandType::WriteCloudFlareDomainARecord { model } => {
                    if let Some(params_to_repeat) = &params_to_repeat {
                        for value in &params_to_repeat.values {
                            env_ctx
                                .apply_step_repeat_parameter(
                                    Some((params_to_repeat.name.as_str(), value)),
                                    &logs,
                                )
                                .await;

                            super::execute_cloud_flare_write_domain(&env_ctx, &model, &logs)
                                .await?;
                        }
                    } else {
                        env_ctx.apply_step_repeat_parameter(None, &logs).await;
                        super::execute_cloud_flare_write_domain(&env_ctx, &model, &logs).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
