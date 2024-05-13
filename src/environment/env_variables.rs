use std::collections::HashMap;

use rust_extensions::StrOrString;

use crate::settings::ExternalVariablesModel;

pub struct EnvVariables {
    home_variables: HashMap<String, String>,
    release_file_variables: HashMap<String, String>,
    vars_from_files: HashMap<String, ExternalVariablesModel>,
}

impl EnvVariables {
    pub fn new(
        home_variables: HashMap<String, String>,
        release_file_variables: HashMap<String, String>,
        vars_from_files: HashMap<String, ExternalVariablesModel>,
    ) -> Self {
        Self {
            home_variables,
            release_file_variables,
            vars_from_files,
        }
    }

    pub fn get<'s>(&'s self, key: &str) -> StrOrString<'s> {
        if self.release_file_variables.get(key).is_some() && self.home_variables.get(key).is_some()
        {
            panic!(
                "Variable {} is defined in both home and working yaml files",
                key
            );
        }
        if let Some(value) = self.release_file_variables.get(key) {
            return value.as_str().into();
        }

        if let Some(value) = self.home_variables.get(key) {
            return value.as_str().into();
        }

        for external_vars in self.vars_from_files.values() {
            if let Some(value) = external_vars.vars.get(key) {
                return value.as_str().into();
            }
        }

        if let Ok(value) = std::env::var(key) {
            println!("Read Variable {} is defined in environment", key);
            return value.into();
        }

        panic!("Variable {} not found", key);
    }
}
