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
    let envs =
        crate::execution::get_execution_args_list(action.app.clone(), &input_data.env, logs).await;
    let result = ReleaseDataHttpResponse {
        ids: envs
            .ids
            .into_iter()
            .map(|itm| IdGroupHttpModel {
                category: itm.category,
                ids: itm
                    .ids
                    .into_iter()
                    .map(|(id, exclude_features)| ReleaseStepHttpModel {
                        id,
                        exclude_features,
                    })
                    .collect(),
            })
            .collect(),
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
    pub ids: Vec<IdGroupHttpModel>,
    pub labels: Vec<String>,
}

#[derive(serde::Serialize, Debug, MyHttpObjectStructure)]
pub struct IdGroupHttpModel {
    pub category: String,
    pub ids: Vec<ReleaseStepHttpModel>,
}

#[derive(serde::Serialize, Debug, MyHttpObjectStructure)]
pub struct ReleaseStepHttpModel {
    pub id: String,
    pub exclude_features: Vec<String>,
}
