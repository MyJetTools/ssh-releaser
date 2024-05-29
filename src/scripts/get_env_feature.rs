use std::sync::Arc;

use crate::{app::AppContext, execution::ExecuteLogsContainer};

pub async fn get_env_feature(app: &Arc<AppContext>, id: String) -> Option<String> {
    let app = app.clone();
    let result = tokio::spawn(async move {
        let logs = Arc::new(ExecuteLogsContainer::new());
        app.global_settings.get_env_settings(&id, &logs).await
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

    result.unwrap().feature
}
