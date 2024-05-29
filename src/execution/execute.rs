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
) -> Result<(), ExecuteCommandError> {
    let env_ctx = app.global_settings.get_env_settings(&env, &logs).await?;

    for step in env_ctx.get_execution_steps() {
        if !env_ctx
            .execute_me(&logs, step, &args, env_ctx.get_feature())
            .await
        {
            continue;
        }

        logs.start_process(step.id.to_string()).await;

        let script_model = ScriptModel::from_step(step, &env_ctx, &logs).await?;

        for remote_command in script_model.get_commands() {
            if let Some(name) = remote_command.name.as_ref() {
                logs.write_log(format!("Executing Script step: {}", name))
                    .await;
            }

            match remote_command.get_remote_command_type(script_model.get_current_path()) {
                settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                    execute_commands(&env_ctx, &script_model, &ssh, commands.as_slice(), &logs)
                        .await?;
                }

                settings::RemoteCommandType::UploadFile { ssh, params, file } => {
                    upload_file(&env_ctx, &script_model, params, &ssh, file, &logs).await?;
                }

                settings::RemoteCommandType::PostRequest { ssh, data } => {
                    execute_post_request(&env_ctx, &script_model, &ssh, &data, &logs).await?;
                }

                settings::RemoteCommandType::GetRequest { ssh, data } => {
                    execute_get_request(&env_ctx, &script_model, &ssh, &data, &logs).await?;
                }
                settings::RemoteCommandType::FromTemplate {
                    from_file,
                    params,
                    script_file_path,
                } => {
                    crate::execute_from_template::execute_from_template(
                        &env_ctx,
                        from_file,
                        script_file_path,
                        params,
                        &logs,
                    )
                    .await?;
                }

                settings::RemoteCommandType::WriteCloudFlareDomainARecord(model) => {
                    super::execute_cloud_flare_write_domain(&env_ctx, model, &logs).await?;
                }
            }
        }
    }

    Ok(())
}
