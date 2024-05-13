use std::sync::Arc;

use my_http_server::{
    macros::{MyHttpInput, MyHttpObjectStructure},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/release/all",
    controller: "ReleaseData",
    description: "Get release data",
    summary: "Returns release data",
    input_data: GetReleaseDataHttpInput,
    result:[
        {status_code: 200, description: "Rows", model: "ReleaseDataHttpResponse"},
    ]
)]
pub struct GetReleaseDataAction {
    app: Arc<AppContext>,
}

impl GetReleaseDataAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetReleaseDataAction,
    input_data: GetReleaseDataHttpInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let logs = Arc::new(crate::execution::ExecuteLogsContainer::new());
    let envs = crate::execution::get_execution_args_list(&action.app, &input_data.env, logs).await;
    let result = ReleaseDataHttpResponse {
        ids: envs.ids,
        labels: envs.labels,
    };
    HttpOutput::as_json(result).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct GetReleaseDataHttpInput {
    #[http_query(description = "Environment")]
    pub env: String,
}

#[derive(serde::Serialize, Debug, MyHttpObjectStructure)]
pub struct ReleaseDataHttpResponse {
    pub ids: Vec<String>,
    pub labels: Vec<String>,
}
