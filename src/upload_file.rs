use std::{collections::HashMap, time::Duration};

use crate::{
    app::AppContext,
    settings::{ScriptModel, UploadFileModel},
};

pub async fn upload_file(
    app: &AppContext,
    script_model: &ScriptModel,
    params: Option<HashMap<String, String>>,
    ssh: &str,
    file: UploadFileModel,
) {
    let local_file =
        crate::scripts::populate_variables(app, Some(script_model), &file.local_file.as_str())
            .await;

    let mut content = crate::scripts::load_file_and_populate_placeholders(
        app,
        Some(script_model),
        local_file.as_str(),
    )
    .await;

    if let Some(params) = params {
        let env = UploadFileEnvironment::new(params);
        content = crate::scripts::populate_variables_after_loading_from_file(
            app,
            Some(&env),
            content,
            "*{",
        )
    }

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

pub struct UploadFileEnvironment {
    params: HashMap<String, String>,
}

impl UploadFileEnvironment {
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }
}

impl crate::script_environment::ScriptEnvironment for UploadFileEnvironment {
    fn get_var(&self, name: &str) -> Option<&str> {
        let result = self.params.get(name)?;
        Some(result)
    }

    fn get_current_path<'s>(&'s self) -> Option<crate::file_path::FilePathRef<'s>> {
        None
    }
}
