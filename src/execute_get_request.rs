use std::str::FromStr;

use hyper::Uri;

use crate::{
    app::AppContext,
    http_over_ssh::Http1Client,
    scripts,
    settings::{GetDataModel, ScriptModel},
};

pub async fn execute_get_request(
    app: &AppContext,
    script: &ScriptModel,
    ssh: &str,
    get_request: &GetDataModel,
) {
    let ssh_credentials = app.get_ssh_credentials(ssh);

    let url = scripts::populate_variables(app, Some(script), get_request.url.as_str()).await;

    let remote_uri = Uri::from_str(url.as_str()).unwrap();

    //    println!("Content: {}", content);

    let http_client = Http1Client::connect(&ssh_credentials, &remote_uri)
        .await
        .unwrap();

    let (status_code, text) = http_client
        .get(remote_uri, &get_request.headers)
        .await
        .unwrap();

    println!("Status code: {}", status_code);
    println!("text: {}", text);
}
