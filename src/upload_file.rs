use std::time::Duration;

use crate::{app::AppContext, settings::UploadFileModel};

pub async fn upload_file(app: &AppContext, ssh: &str, file: UploadFileModel) {
    let content = crate::scripts::load_file_and_populate_placeholders(app, &file.local_path).await;

    let session = app.get_ssh_session(ssh).await;

    let result = session
        .upload_file(
            &file.remote_path,
            content.as_bytes(),
            file.mode,
            Duration::from_secs(30),
        )
        .await
        .unwrap();

    println!("File uploaded with result: {}", result);
}
