use std::collections::HashMap;

use serde::*;

use super::StepModel;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSettingsModel {
    pub vars: HashMap<String, String>,
    pub var_files: Option<Vec<String>>,
    pub steps: Vec<StepModel>,
    pub execute_steps: Vec<String>,
}

impl ReleaseSettingsModel {
    pub fn execute_me(&self, step: &StepModel) -> bool {
        for execute_step in self.execute_steps.iter() {
            if execute_step == "*" {
                return true;
            }

            if execute_step == &step.id {
                return true;
            }

            if let Some(labels) = step.labels.as_ref() {
                for label in labels {
                    if label == execute_step {
                        return true;
                    }
                }
            }
        }

        false
    }
}
