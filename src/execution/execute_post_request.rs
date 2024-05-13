use std::{str::FromStr, sync::Arc};

use hyper::Uri;

use crate::{
    environment::EnvContext,
    http_over_ssh::Http1Client,
    scripts,
    settings::{PostDataModel, ScriptModel},
};

use super::{ExecuteCommandError, ExecuteLogsContainer};

pub async fn execute_post_request(
    env_settings: &EnvContext,
    script: &ScriptModel,
    ssh: &str,
    post_request: &PostDataModel,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<(), ExecuteCommandError> {
    let ssh_credentials = env_settings.get_ssh_credentials(ssh);

    let url =
        scripts::populate_variables(env_settings, Some(script), post_request.url.as_str()).await;

    let remote_uri = Uri::from_str(url.as_str()).unwrap();

    let content = get_body(env_settings, script, &post_request).await;

    //    println!("Content: {}", content);

    let http_client = Http1Client::connect(&ssh_credentials, &remote_uri).await?;

    let (status_code, text) = http_client
        .post(remote_uri, content.into_bytes(), &post_request.headers)
        .await?;

    logs.write_log(format!("Status code: {}", status_code))
        .await;
    logs.write_log(format!("text: {}", text)).await;

    Ok(())
}

async fn get_body(
    env_settings: &EnvContext,
    script: &ScriptModel,
    model: &PostDataModel,
) -> String {
    if let Some(body) = model.body.as_ref() {
        if model.raw_content() {
            return body.clone();
        }
        return crate::scripts::populate_variables(env_settings, Some(script), body)
            .await
            .to_string();
    }

    if let Some(body_path) = model.body_path.as_ref() {
        let content = crate::scripts::load_file_and_populate_placeholders(
            env_settings,
            Some(script),
            body_path,
        )
        .await;

        return content;
    }

    panic!("Post request must have either 'body' or 'body_path' property");
}
