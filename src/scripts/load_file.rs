use crate::app::AppContext;

pub async fn load_file_and_populate_placeholders(app: &AppContext, file_name: &str) -> String {
    let file_name = app.settings.get_file_name(file_name);

    println!("Loading file: {}", file_name);

    let content = tokio::fs::read_to_string(file_name).await.unwrap();

    return crate::scripts::populate_variables(app, content.as_str())
        .await
        .to_string();
}

pub async fn load_file(app: &AppContext, file_name: &str) -> String {
    let file_name = app.settings.get_file_name(file_name);

    println!("Loading file: {}", file_name);

    return tokio::fs::read_to_string(file_name).await.unwrap();
}
