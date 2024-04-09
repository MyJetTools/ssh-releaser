use rust_extensions::StrOrString;

use crate::app::AppContext;

pub const PLACEHOLDER_OPEN_TOKEN: &str = "${";
pub const PLACEHOLDER_CLOSE_TOKEN: &str = "}";

pub fn populate_variables<'s>(app: &AppContext, src: &'s str) -> StrOrString<'s> {
    let index = src.find(PLACEHOLDER_OPEN_TOKEN);

    if index.is_none() {
        return src.into();
    }
    let mut result = String::new();

    for token in rust_extensions::placeholders::PlaceholdersIterator::new(
        src,
        PLACEHOLDER_OPEN_TOKEN,
        PLACEHOLDER_CLOSE_TOKEN,
    ) {
        match token {
            rust_extensions::placeholders::ContentToken::Text(text) => result.push_str(text),
            rust_extensions::placeholders::ContentToken::Placeholder(placeholder) => {
                if let Some(value) = app.release_settings.vars.get(placeholder) {
                    result.push_str(value);
                } else {
                    if let Ok(value) = std::env::var(placeholder) {
                        result.push_str(value.as_str());
                    } else {
                        panic!("Variable {} not found", placeholder)
                    }
                }
            }
        }
    }

    result.into()
}
