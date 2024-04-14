use crate::{app::AppContext, script_environment::ScriptEnvironment};

use super::PopulateVariablesProcessing;

pub const PLACEHOLDER_CLOSE_TOKEN: &str = "}";

pub fn populate_variables_after_loading_from_file(
    app: &AppContext,
    script_env: Option<&impl ScriptEnvironment>,
    src: String,
    open_token: &'static str,
) -> String {
    let index = src.find(open_token);

    if index.is_none() {
        return src;
    }
    let mut result = String::new();

    for token in rust_extensions::placeholders::PlaceholdersIterator::new(
        src.as_str(),
        open_token,
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
                    let (placeholder_to_process, processing) = match placeholder.find(":") {
                        Some(index) => {
                            let placeholder_to_process = &placeholder[..index];
                            let processing = PopulateVariablesProcessing::new(
                                &placeholder[index + 1..],
                                placeholder,
                            );
                            (placeholder_to_process, processing)
                        }
                        None => (placeholder, PopulateVariablesProcessing::empty()),
                    };

                    let content = app.get_env_variable(script_env, placeholder_to_process);

                    if processing.has_url_encoded() {
                        let url_encoded = super::convert_url_encoded(content.as_str());
                        result.push_str(url_encoded.as_str());
                    } else {
                        result.push_str(content.as_str());
                    }
                }
            }
        }
    }

    result.into()
}
