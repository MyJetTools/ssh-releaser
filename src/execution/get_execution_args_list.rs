use std::sync::Arc;

use crate::app::AppContext;

use super::ExecuteLogsContainer;

pub struct ExecutionArgsList {
    pub ids: Vec<String>,
    pub labels: Vec<String>,
    pub err: Option<String>,
}

pub async fn get_execution_args_list(
    app: &AppContext,
    env: &str,
    logs: Arc<ExecuteLogsContainer>,
) -> ExecutionArgsList {
    let env_settings = match app.global_settings.get_env_settings(env, &logs).await {
        Ok(env_settings) => env_settings,
        Err(err) => {
            let err_str = format!("Error getting env settings: {:?}", err);
            logs.write_error(err).await;
            return ExecutionArgsList {
                ids: Vec::new(),
                labels: Vec::new(),
                err: Some(err_str),
            };
        }
    };

    let mut ids = Vec::new();

    let mut labels_result = Vec::new();

    for step_model in env_settings.get_execution_steps() {
        ids.push(step_model.id.clone());

        if let Some(labels) = step_model.labels.as_ref() {
            for label in labels {
                if !labels_result.contains(label) {
                    labels_result.push(label.to_string());
                }
            }
        }
    }

    ExecutionArgsList {
        ids,
        labels: labels_result,
        err: None,
    }
}
