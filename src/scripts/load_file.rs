use crate::app::AppContext;

pub async fn load_file(app: &AppContext, file_name: &str, populate_placeholders: bool) -> String {
    let file_name = app.settings.get_file_name(file_name);

    println!("File to upload: {}", file_name);

    let content = tokio::fs::read_to_string(file_name).await.unwrap();

    if populate_placeholders {
        return crate::scripts::populate_variables(app, content.as_str()).to_string();
    }

    content
}
