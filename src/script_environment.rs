pub trait ScriptEnvironment {
    fn get_var(&self, name: &str) -> Option<&str>;
    fn get_current_path(&self) -> Option<&str>;
}
