use std::collections::HashMap;

use crate::{file_path::FilePathRef, script_environment::ScriptEnvironment};

pub struct ReadingFromTemplateEnvironment {
    vars: Option<HashMap<String, String>>,
}

impl ReadingFromTemplateEnvironment {
    pub fn new(vars: Option<HashMap<String, String>>) -> Self {
        ReadingFromTemplateEnvironment { vars }
    }
}

impl ScriptEnvironment for ReadingFromTemplateEnvironment {
    fn get_var(&self, name: &str) -> Option<&str> {
        let vars = self.vars.as_ref()?;

        vars.get(name).map(|v| v.as_str())
    }

    fn get_current_path<'s>(&'s self) -> Option<FilePathRef<'s>> {
        None
    }
}
