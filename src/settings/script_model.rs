use crate::{app::AppContext, file_path::FilePathRef, script_environment::ScriptEnvironment};

use super::{RemoteCommand, ScriptFromFileModel, ScriptFromSettingsModel, StepModel};

pub enum ScriptModel {
    FromSettings(ScriptFromSettingsModel),
    FromFile(ScriptFromFileModel),
}

impl ScriptModel {
    pub async fn from_step(step_model: &StepModel, app: &AppContext) -> ScriptModel {
        if let Some(commends) = step_model.script.as_ref() {
            return ScriptModel::FromSettings(ScriptFromSettingsModel {
                commands: commends.clone(),
            });
        }

        if let Some(from_file) = step_model.from_file.as_ref() {
            let script_env: Option<&ScriptModel> = None;
            let (file_content, file_name) =
                crate::scripts::load_file(app, script_env, from_file).await;

            return ScriptModel::from_content(file_content.as_str(), file_name.get_file_path());
        }

        panic!("Please specify either script or from_file in the step model")
    }

    pub fn from_content(file_content: &str, file_path: FilePathRef<'_>) -> ScriptModel {
        let mut result: ScriptFromFileModel = serde_yaml::from_str(file_content).unwrap();

        result.current_path = Some(file_path.to_owned());
        return ScriptModel::FromFile(result);
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
