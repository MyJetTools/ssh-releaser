use std::sync::Arc;

use my_http_server::{
    macros::{MyHttpInput, MyHttpInputObjectStructure},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};
use serde::{Deserialize, Serialize};

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/release/logs",
    controller: "ReleaseData",
    description: "Get logs info",
    summary: "Get logs info",
    input_data: GetLogsInputData,
    result:[
        {status_code: 200, description: "Rows", model: "GetLogsHttpResponse"},
    ]
)]
pub struct GetLogsInfoAction {
    app: Arc<AppContext>,
}

impl GetLogsInfoAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetLogsInfoAction,
    input_data: GetLogsInputData,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let container = action.app.get_container(&input_data.id).await;

    if container.is_none() {
        return Err(HttpFailResult::as_not_found(
            "Container not found".to_string(),
            false,
        ));
    }

    let container = container.unwrap();

    let result = GetLogsHttpResponse {
        html: container.get_as_html().await,
        finished: container.get_finished(),
    };
    HttpOutput::as_json(result).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct GetLogsInputData {
    #[http_query(description = "Id of process")]
    pub id: String,
}

#[derive(MyHttpInputObjectStructure, Serialize, Deserialize)]
pub struct GetLogsHttpResponse {
    pub html: String,
    pub finished: bool,
}
