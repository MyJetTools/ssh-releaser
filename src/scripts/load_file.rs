use std::sync::Arc;

use crate::{environment::EnvContext, execution::*, file_name::FileName};

pub async fn load_file_and_populate_placeholders(
    env_ctx: &EnvContext,
    script_env: Option<&impl ScriptEnvironment>,
    file_name: &str,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<String, ExecuteCommandError> {
    let file_name = env_ctx.get_file_name(script_env, file_name);

    logs.write_log(format!("Loading file: {}", file_name.as_str()))
        .await;

    let content = tokio::fs::read_to_string(file_name.as_str()).await.unwrap();

    let result = crate::scripts::populate_variables(env_ctx, script_env, content.as_str(), logs)
        .await?
        .to_string();

    Ok(result)
}

pub async fn load_file(
    env_ctx: &EnvContext,
    script_env: Option<&impl ScriptEnvironment>,
    file_name: &str,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<(String, FileName), ExecuteCommandError> {
    let file_name = env_ctx.get_file_name(script_env, file_name);

    let result = file_name.load_content_as_string(&env_ctx.app, logs).await?;

    Ok((result, file_name))
}
