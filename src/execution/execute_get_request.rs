use std::{str::FromStr, sync::Arc};

use hyper::Uri;

use crate::{
    environment::EnvContext,
    http_over_ssh::Http1Client,
    scripts,
    settings::{GetDataModel, ScriptModel},
};

use super::{ExecuteCommandError, ExecuteLogsContainer};

pub async fn execute_get_request(
    env_settings: &EnvContext,
    script: &ScriptModel,
    ssh: &str,
    get_request: &GetDataModel,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<(), ExecuteCommandError> {
    let ssh_credentials = env_settings.get_ssh_credentials(ssh)?;

    let url =
        scripts::populate_variables(env_settings, Some(script), get_request.url.as_str(), logs)
            .await?;

    let remote_uri = Uri::from_str(url.as_str()).unwrap();

    //    println!("Content: {}", content);

    let http_client = Http1Client::connect(&ssh_credentials, &remote_uri, logs).await?;

    let (status_code, text) = http_client.get(remote_uri, &get_request.headers).await?;

    logs.write_log(format!("Status code: {}", status_code))
        .await;
    logs.write_log(format!("text: {}", text)).await;

    Ok(())
}
