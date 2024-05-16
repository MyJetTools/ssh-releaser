use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/env/list",
    controller: "Rows",
    description: "Returns list of active environments",
    summary: "Returns list of active environments",
    result:[
        {status_code: 200, description: "Rows", model: "Vec<String>"},
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
    let envs = action.app.global_settings.get_envs();
    HttpOutput::as_json(envs).into_ok_result(false)
}
