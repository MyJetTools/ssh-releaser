use crate::{app::AppContext, file_name::FileName, script_environment::ScriptEnvironment};

pub async fn load_file_and_populate_placeholders(
    app: &AppContext,
    script_env: Option<&impl ScriptEnvironment>,
    file_name: &str,
) -> String {
    let file_name = app.settings.get_file_name(script_env, file_name);

    println!("Loading file: {}", file_name.as_str());

    let content = tokio::fs::read_to_string(file_name.as_str()).await.unwrap();

    return crate::scripts::populate_variables(app, script_env, content.as_str())
        .await
        .to_string();
}

pub async fn load_file(
    app: &AppContext,
    script_env: Option<&impl ScriptEnvironment>,
    file_name: &str,
) -> (String, FileName) {
    let file_name = app.settings.get_file_name(script_env, file_name);

    println!("Loading file: {}", file_name.as_str());

    let result = tokio::fs::read_to_string(file_name.as_str()).await.unwrap();

    (result, file_name)
}
