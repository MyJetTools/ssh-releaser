use std::sync::Arc;

use crate::{
    environment::EnvContext,
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
    let ssh_credentials = env_settings.get_ssh_credentials(ssh)?;

    let url =
        scripts::populate_variables(env_settings, Some(script), post_request.url.as_str(), logs)
            .await?;

    let content = get_body(env_settings, script, &post_request, logs).await?;

    println!("Post body: {}", content);

    //    println!("Content: {}", content);

    let content_type = detect_content_type(content.as_bytes());

    println!("Content type: {}", content_type);

    let mut fl_url_response = flurl::FlUrl::new(url.as_str())
        .set_ssh_credentials(ssh_credentials)
        .do_not_reuse_connection()
        .with_header("content-type", content_type)
        .update_mode(flurl::FlUrlMode::Http1Hyper)
        .post(Some(content.into_bytes()))
        .await
        .unwrap();

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

async fn get_body(
    env_settings: &EnvContext,
    script: &ScriptModel,
    model: &PostDataModel,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<String, ExecuteCommandError> {
    if let Some(body) = model.body.as_ref() {
        if model.raw_content() {
            return Ok(body.clone());
        }
        let result =
            crate::scripts::populate_variables(env_settings, Some(script), body, logs).await?;
        return Ok(result.to_string());
    }

    if let Some(body_path) = model.body_path.as_ref() {
        let content = crate::scripts::load_file_and_populate_placeholders(
            env_settings,
            Some(script),
            body_path,
            logs,
        )
        .await;

        return content;
    }

    Err(
        "Post request must have either 'body' or 'body_path' property"
            .to_string()
            .into(),
    )
}

fn detect_content_type(body: &[u8]) -> &'static str {
    for b in body {
        let b = *b;
        if b <= 32 {
            continue;
        }

        if b == b'[' || b == b'{' {
            return "application/json";
        }

        break;
    }

    "x-www-form-urlencoded"
}
