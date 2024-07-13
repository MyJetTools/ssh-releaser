use std::sync::Arc;

use crate::{
    environment::EnvContext,
    execution::{ExecuteCommandError, ExecuteLogsContainer, ScriptEnvironment},
    file_path::FilePathRef,
};

use super::{RemoteCommand, ScriptFromFileModel, ScriptFromSettingsModel, StepModel};

pub enum ScriptModel {
    FromSettings(ScriptFromSettingsModel),
    FromFile(ScriptFromFileModel),
}

impl ScriptModel {
    pub async fn from_step(
        step_model: &StepModel,
        env_ctx: &EnvContext,
        logs: &Arc<ExecuteLogsContainer>,
    ) -> Result<ScriptModel, ExecuteCommandError> {
        if let Some(commends) = step_model.script.as_ref() {
            return Ok(ScriptModel::FromSettings(ScriptFromSettingsModel {
                commands: commends.clone(),
            }));
        }

        if let Some(from_file) = step_model.from_file.as_ref() {
            let script_env: Option<&ScriptModel> = None;
            let (file_content, file_name) =
                crate::scripts::load_file(env_ctx, script_env, from_file, logs).await?;

            return ScriptModel::from_content(file_content.as_str(), file_name.get_file_path());
        }

        Err(
            "Please specify either script or from_file in the step model"
                .to_string()
                .into(),
        )
    }

    pub fn from_content(
        file_content: &str,
        file_path: FilePathRef<'_>,
    ) -> Result<ScriptModel, ExecuteCommandError> {
        let mut result: ScriptFromFileModel = match serde_yaml::from_str(file_content) {
            Ok(result) => result,
            Err(err) => return Err(err.to_string().into()),
        };

        result.current_path = Some(file_path.to_owned());
        return Ok(ScriptModel::FromFile(result));
    }

    pub fn get_commands(&self) -> &[RemoteCommand] {
        match self {
            ScriptModel::FromSettings(from_settings) => from_settings.commands.as_slice(),
            ScriptModel::FromFile(from_file) => from_file.script.as_slice(),
        }
    }
}

impl ScriptEnvironment for ScriptModel {
    fn get_var(&self, key: &str) -> Option<&str> {
        match self {
            ScriptModel::FromSettings(_) => {}
            ScriptModel::FromFile(from_file) => {
                if let Some(vars) = from_file.vars.as_ref() {
                    return vars.get(key).map(|itm| itm.as_str());
                }
            }
        }

        None
    }

    fn get_current_path<'s>(&'s self) -> Option<FilePathRef<'s>> {
        match self {
            ScriptModel::FromSettings(_) => None,
            ScriptModel::FromFile(model) => {
                let result = model.current_path.as_ref()?;
                Some(result.as_ref())
            }
        }
    }
}
