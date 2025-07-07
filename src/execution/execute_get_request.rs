use std::sync::Arc;

use crate::{
    environment::EnvContext,
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

    //    let remote_uri = Uri::from_str(url.as_str()).unwrap();

    //    println!("Content: {}", content);

    let mut fl_url_response = flurl::FlUrl::new(url.as_str())
        .set_ssh_credentials(ssh_credentials)
        .get()
        .await
        .unwrap();

    //let http_client = Http1Client::connect(&ssh_credentials, &remote_uri, logs).await?;

    //let (status_code, text) = http_client.get(remote_uri, &get_request.headers).await?;

    logs.write_log(format!(
        "Status code: {}",
        fl_url_response.get_status_code()
    ))
    .await;
    logs.write_log(format!(
        "text: {}",
        fl_url_response.get_body_as_str().await.unwrap()
    ))
    .await;

    Ok(())
}
