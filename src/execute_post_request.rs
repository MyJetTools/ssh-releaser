use std::str::FromStr;

use hyper::Uri;

use crate::{app::AppContext, http_over_ssh::Http1Client, scripts, settings::PostDataModel};

pub async fn execute_post_request(app: &AppContext, ssh: &str, post_request: &PostDataModel) {
    let ssh_credentials = app.get_ssh_credentials(ssh);

    let url = scripts::populate_variables(app, post_request.url.as_str());

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
        return crate::scripts::populate_for_post_request(app, body.clone()).await;
    }

    if let Some(body_path) = model.body_path.as_ref() {
        let content = crate::scripts::load_file(app, body_path, false).await;
        return crate::scripts::populate_for_post_request(app, content).await;
    }

    panic!("Post request must have either 'body' or 'body_path' property");
}
