use std::sync::Arc;

use crate::{app::AppContext, execution::ExecuteLogsContainer};

pub async fn get_env_features(
    app: &Arc<AppContext>,
    product_code: String,
    id: String,
) -> Option<Vec<String>> {
    let app = app.clone();
    let result = tokio::spawn(async move {
        let logs = Arc::new(ExecuteLogsContainer::new());

        app.clone()
            .global_settings
            .get_env_settings(app, &product_code, &id, &logs)
            .await
    })
    .await;

    if result.is_err() {
        return None;
    }

    let result = result.unwrap();

    if let Err(err) = &result {
        println!("Error: {:?}", err);
        return None;
    }

    result.unwrap().features
}
