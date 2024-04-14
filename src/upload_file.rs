use std::time::Duration;

use crate::{
    app::AppContext,
    settings::{ScriptModel, UploadFileModel},
};

pub async fn upload_file(
    app: &AppContext,
    script_model: &ScriptModel,
    ssh: &str,
    file: UploadFileModel,
) {
    let local_file =
        crate::scripts::populate_variables(app, Some(script_model), &file.local_file.as_str())
            .await;

    let content = crate::scripts::load_file_and_populate_placeholders(
        app,
        Some(script_model),
        local_file.as_str(),
    )
    .await;

    let session = app.get_ssh_session(ssh).await;

    let remote_file =
        crate::scripts::populate_variables(app, Some(script_model), file.remote_file.as_str())
            .await;

    println!("Uploading file to remote path: {}", remote_file.as_str());

    let result = session
        .upload_file(
            remote_file.as_str(),
            content.as_bytes(),
            file.mode,
            Duration::from_secs(30),
        )
        .await
        .unwrap();

    println!("File uploaded with result: {}", result);
}
