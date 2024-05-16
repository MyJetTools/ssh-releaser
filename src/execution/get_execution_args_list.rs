use std::sync::Arc;

use crate::app::AppContext;

use super::ExecuteLogsContainer;

pub struct IdGroup {
    pub category: String,
    pub ids: Vec<String>,
}

pub struct ExecutionArgsList {
    pub ids: Vec<IdGroup>,
    pub labels: Vec<String>,
    pub err: Option<String>,
    pub features: Vec<String>,
}

const NONE_CATEGORY_NAME: &str = "---";

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
                features: Vec::new(),
                err: Some(err_str),
            };
        }
    };

    let mut ids = Vec::new();

    let mut labels_result = Vec::new();

    let mut features = Vec::new();

    for step_model in env_settings.get_execution_steps() {
        let category_index = match &step_model.category {
            Some(category) => ids
                .iter()
                .position(|id_group: &IdGroup| id_group.category.as_str() == category),
            None => ids
                .iter()
                .position(|id_group: &IdGroup| id_group.category.as_str() == NONE_CATEGORY_NAME),
        };

        match category_index {
            Some(index) => {
                ids[index].ids.push(step_model.id.clone());
            }
            None => {
                ids.push(IdGroup {
                    category: step_model
                        .category
                        .clone()
                        .unwrap_or(NONE_CATEGORY_NAME.to_string()),
                    ids: vec![step_model.id.clone()],
                });
            }
        }

        if let Some(labels) = step_model.labels.as_ref() {
            for label in labels {
                if !labels_result.contains(label) {
                    labels_result.push(label.to_string());
                }
            }
        }

        if let Some(features_to_scan) = step_model.features_exclude.as_ref() {
            for feature in features_to_scan {
                if !features.contains(feature) {
                    features.push(feature.to_string());
                }
            }
        }

        if let Some(features_to_scan) = step_model.features_include.as_ref() {
            for feature in features_to_scan {
                if !features.contains(feature) {
                    features.push(feature.to_string());
                }
            }
        }
    }

    ExecutionArgsList {
        ids,
        labels: labels_result,
        features: features,
        err: None,
    }
}
