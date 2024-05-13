use crate::app::AppContext;

pub struct ExecutionArgsList {
    pub ids: Vec<String>,
    pub labels: Vec<String>,
}

pub async fn get_execution_args_list(app: &AppContext, env: &str) -> ExecutionArgsList {
    let env_settings = app.global_settings.get_env_settings(env).await;

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
    }
}
