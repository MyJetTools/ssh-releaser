use crate::{app::AppContext, script_environment::ScriptEnvironment};

pub const PLACEHOLDER_OPEN_TOKEN: &str = "${";
pub const PLACEHOLDER_CLOSE_TOKEN: &str = "}";

pub fn populate_variables_after_loading_from_file(
    app: &AppContext,
    script_env: Option<&impl ScriptEnvironment>,
    src: String,
) -> String {
    let index = src.find(PLACEHOLDER_OPEN_TOKEN);

    if index.is_none() {
        return src;
    }
    let mut result = String::new();

    for token in rust_extensions::placeholders::PlaceholdersIterator::new(
        src.as_str(),
        PLACEHOLDER_OPEN_TOKEN,
        PLACEHOLDER_CLOSE_TOKEN,
    ) {
        match token {
            rust_extensions::placeholders::ContentToken::Text(text) => result.push_str(text),
            rust_extensions::placeholders::ContentToken::Placeholder(placeholder) => {
                if placeholder.starts_with("$") {
                    result.push_str("${");
                    result.push_str(placeholder[1..].as_ref());
                    result.push('}');
                } else {
                    let value = app.get_env_variable(script_env, placeholder);
                    result.push_str(value.as_str());
                }
            }
        }
    }

    result.into()
}
