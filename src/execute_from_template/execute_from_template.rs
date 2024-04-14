use std::collections::HashMap;

use crate::{
    app::AppContext,
    execute_from_template::ReadingFromTemplateEnvironment,
    file_path::FilePath,
    script_environment::ScriptEnvironment,
    settings::{self, ScriptModel},
};

pub async fn execute_from_template(
    app: &AppContext,
    from_file: String,
    script_file_path: FilePath,
    params: Option<HashMap<String, String>>,
) {
    let script_env: Option<&ScriptModel> = None;
    let file_name = app.settings.get_file_name(script_env, from_file.as_str());

    println!("Loading template from file: {}", file_name.as_str());
    let content = match tokio::fs::read_to_string(file_name.as_str()).await {
        Ok(result) => result,
        Err(e) => {
            panic!("Error reading file {}. Err: {:?}", from_file, e)
        }
    };

    let loading_template_env = ReadingFromTemplateEnvironment::new(params);
    let content = crate::scripts::populate_variables_after_loading_from_file(
        app,
        Some(&loading_template_env),
        content,
        "*{",
    );

    let script_model = ScriptModel::from_content(content.as_str(), script_file_path.as_ref());

    for remote_command in script_model.get_commands() {
        println!("-----------------");
        if let Some(name) = remote_command.name.as_ref() {
            println!("Executing Script step: {}", name);
        }

        match remote_command.get_remote_command_type(script_model.get_current_path()) {
            settings::RemoteCommandType::ExecuteCommands { ssh, commands } => {
                crate::execute_commands::execute_commands(
                    &app,
                    &script_model,
                    &ssh,
                    commands.as_slice(),
                )
                .await;
            }

            settings::RemoteCommandType::UploadFile { ssh, file } => {
                crate::upload_file::upload_file(&app, &script_model, &ssh, file).await;
            }

            settings::RemoteCommandType::PostRequest { ssh, data } => {
                crate::execute_post_request::execute_post_request(&app, &script_model, &ssh, &data)
                    .await;
            }

            settings::RemoteCommandType::GetRequest { ssh, data } => {
                crate::execute_get_request::execute_get_request(&app, &script_model, &ssh, &data)
                    .await;
            }
            settings::RemoteCommandType::FromTemplate {
                from_file,
                params,
                script_file_path,
            } => {
                panic!(
                    "Nested templates are not supported yet. Please check file: {} with params: {:?}. Script file name: {}",
                    from_file, params, script_file_path.as_str()
                )
            }
        }
    }
}