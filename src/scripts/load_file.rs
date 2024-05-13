use crate::{environment::EnvContext, execution::*, file_name::FileName};

pub async fn load_file_and_populate_placeholders(
    settings: &EnvContext,
    script_env: Option<&impl ScriptEnvironment>,
    file_name: &str,
) -> String {
    let file_name = settings.get_file_name(script_env, file_name);

    println!("Loading file: {}", file_name.as_str());

    let content = tokio::fs::read_to_string(file_name.as_str()).await.unwrap();

    return crate::scripts::populate_variables(settings, script_env, content.as_str())
        .await
        .to_string();
}

pub async fn load_file(
    settings: &EnvContext,
    script_env: Option<&impl ScriptEnvironment>,
    file_name: &str,
) -> (String, FileName) {
    let file_name = settings.get_file_name(script_env, file_name);

    let result = file_name.load_content_as_string().await;

    (result, file_name)
}
