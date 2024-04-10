use core::panic;

use crate::app::AppContext;

pub const PLACEHOLDER_OPEN_TOKEN: &str = "${";
pub const PLACEHOLDER_CLOSE_TOKEN: &str = "}";

pub fn populate_variables_after_loading_from_file(app: &AppContext, src: String) -> String {
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
                } else if let Some(value) = app.release_settings.vars.get(placeholder) {
                    result.push_str(value);
                } else if let Ok(value) = std::env::var(placeholder) {
                    result.push_str(value.as_str());
                } else {
                    panic!("Variable {} not found", placeholder);
                }
            }
        }
    }

    result.into()
}
