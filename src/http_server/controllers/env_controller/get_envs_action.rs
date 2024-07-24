use std::sync::Arc;

use my_http_server::{
    macros::MyHttpObjectStructure, HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};
use serde::Serialize;

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/env/list",
    controller: "Rows",
    description: "Returns list of active environments",
    summary: "Returns list of active environments",
    result:[
        {status_code: 200, description: "Rows", model: "Vec<EnvironmentHttpOutput>"},
    ]
)]
pub struct GetEnvironmentsAction {
    app: Arc<AppContext>,
}

impl GetEnvironmentsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetEnvironmentsAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let mut response: Vec<EnvironmentHttpOutput> = Vec::new();

    for env in action.app.global_settings.get_envs() {
        println!("Getting feature for env: {}", env);
        let features = crate::scripts::get_env_features(&action.app, env.clone()).await;
        println!("Got features {:?} for env: {}", features, env);
        response.push(EnvironmentHttpOutput { id: env, features });
    }

    HttpOutput::as_json(response).into_ok_result(false)
}

#[derive(MyHttpObjectStructure, Serialize)]
pub struct EnvironmentHttpOutput {
    pub id: String,
    pub features: Option<Vec<String>>,
}
