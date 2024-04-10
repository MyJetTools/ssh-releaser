use std::str::FromStr;

use hyper::Uri;

use crate::{app::AppContext, http_over_ssh::Http1Client, scripts, settings::PostDataModel};

pub async fn execute_post_request(app: &AppContext, ssh: &str, post_request: &PostDataModel) {
    let ssh_credentials = app.get_ssh_credentials(ssh);

    let url = scripts::populate_variables(app, post_request.url.as_str()).await;

    let remote_uri = Uri::from_str(url.as_str()).unwrap();

    let content = get_body(app, &post_request).await;

    //    println!("Content: {}", content);

    let http_client = Http1Client::connect(&ssh_credentials, &remote_uri)
        .await
        .unwrap();

    let status_code = http_client
        .post(remote_uri, content.into_bytes(), &post_request.headers)
        .await
        .unwrap();

    println!("Status code: {}", status_code);
}

async fn get_body(app: &AppContext, model: &PostDataModel) -> String {
    if let Some(body) = model.body.as_ref() {
        if model.raw_content() {
            return body.clone();
        }
        return crate::scripts::populate_variables(app, body)
            .await
            .to_string();
    }

    if let Some(body_path) = model.body_path.as_ref() {
        let content = crate::scripts::load_file_and_populate_placeholders(app, body_path).await;
        if model.raw_content() {
            return content;
        }
    }

    panic!("Post request must have either 'body' or 'body_path' property");
}
