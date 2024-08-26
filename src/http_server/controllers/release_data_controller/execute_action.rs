use std::sync::Arc;

use my_http_server::{macros::MyHttpInput, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "POST",
    route: "/api/release/execute",
    controller: "ReleaseData",
    description: "Execute release script",
    summary: "Execute release script",
    input_data: ExecuteInputData,
    result:[
        {status_code: 200, description: "Rows", model: "String"},
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
    let logs = Arc::new(crate::execution::ExecuteLogsContainer::new());

    action.app.add_container(logs.clone()).await;

    let logs_spawned = logs.clone();

    let app = action.app.clone();

    tokio::spawn(async move {
        let logs_spawned_squared = logs_spawned.clone();
        let result = tokio::spawn(async move {
            match crate::execution::execute(
                app,
                &input_data.product.as_str(),
                input_data.env,
                input_data.arg,
                logs_spawned_squared.clone(),
            )
            .await
            {
                Ok(_) => {
                    logs_spawned_squared.write_finished_ok().await;
                }
                Err(err) => {
                    logs_spawned_squared.write_finished_err(err).await;
                }
            }
        })
        .await;

        if let Err(err) = result {
            logs_spawned.write_finished_err(format!("{:?}", err)).await;
        }
    });

    HttpOutput::as_text(logs.id.clone()).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct ExecuteInputData {
    #[http_form_data(description = "Product")]
    pub product: String,

    #[http_form_data(description = "Environment")]
    pub env: String,

    #[http_form_data(description = "Arguments")]
    pub arg: String,
}
