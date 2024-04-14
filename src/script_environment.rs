use crate::file_path::FilePathRef;

pub trait ScriptEnvironment {
    fn get_var(&self, name: &str) -> Option<&str>;
    fn get_current_path<'s>(&'s self) -> Option<FilePathRef<'s>>;
}
