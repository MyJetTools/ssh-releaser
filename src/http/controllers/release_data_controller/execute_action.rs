use std::sync::Arc;

use my_http_server::{
    macros::{MyHttpInput, MyHttpObjectStructure},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "POST",
    route: "/api/release/execute",
    controller: "ReleaseData",
    description: "Execute release script",
    summary: "Execute release script",
    input_data: ExecuteInputData,
    result:[
        {status_code: 200, description: "Rows", model: "ReleaseDataHttpResponse"},
    ]
)]
pub struct ExecuteAction {
    app: Arc<AppContext>,
}

impl ExecuteAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ExecuteAction,
    input_data: ExecuteInputData,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let container = crate::execution::execute(&action.app, &input_data.env, &input_data.arg).await;

    let result = container.id.clone();

    action.app.add_container(container).await;
    HttpOutput::as_text(result).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct ExecuteInputData {
    #[http_form_data(description = "Environment")]
    pub env: String,

    #[http_form_data(description = "Arguments")]
    pub arg: String,
}

#[derive(serde::Serialize, Debug, MyHttpObjectStructure)]
pub struct ReleaseDataHttpResponse {
    pub html: Vec<String>,
}
