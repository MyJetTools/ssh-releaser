use rust_extensions::StrOrString;

use crate::{app::AppContext, script_environment::ScriptEnvironment};

use super::PopulateVariablesProcessing;

pub const PLACEHOLDER_OPEN_TOKEN: &str = "${";
pub const PLACEHOLDER_CLOSE_TOKEN: &str = "}";

pub async fn populate_variables<'s>(
    app: &AppContext,
    script_env: Option<&impl ScriptEnvironment>,
    src: &'s str,
) -> StrOrString<'s> {
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

                let populate_placeholders_after_reading_from_file = !processing.has_raw();

                let content = get_placeholder_content(
                    app,
                    script_env,
                    placeholder_to_process,
                    populate_placeholders_after_reading_from_file,
                )
                .await;

                if processing.has_url_encoded() {
                    let url_encoded = convert_url_encoded(content.as_str());
                    result.push_str(url_encoded.as_str());
                } else {
                    result.push_str(content.as_str());
                }
            }
        }
    }

    result.into()
}

async fn get_placeholder_content<'s>(
    app: &'s AppContext,
    script_env: Option<&'s impl ScriptEnvironment>,
    placeholder: &str,
    populate_placeholders_after_reading_from_file: bool,
) -> StrOrString<'s> {
    if placeholder.starts_with("/") || placeholder.starts_with("~") || placeholder.starts_with(".")
    {
        let (mut content, _) = crate::scripts::load_file(app, script_env, placeholder).await;
        if populate_placeholders_after_reading_from_file {
            content = super::populate_variables_after_loading_from_file(
                app,
                script_env,
                content,
                PLACEHOLDER_OPEN_TOKEN,
            );
        }

        return content.into();
    }

    if placeholder.starts_with("$") {
        let mut result = String::new();
        result.push_str("${");
        result.push_str(placeholder[1..].as_ref());
        result.push('}');

        return result.into();
    }

    app.get_env_variable(script_env, placeholder)
}

fn convert_url_encoded(content: &str) -> String {
    let mut result = String::new();

    for c in content.chars() {
        match c {
            ':' => result.push_str("%3A"),
            '\n' => result.push_str("%0A"),
            ' ' => result.push_str("+"),
            '!' => result.push_str("%21"),
            '"' => result.push_str("%22"),
            '#' => result.push_str("%23"),
            '$' => result.push_str("%24"),
            '%' => result.push_str("%25"),
            '&' => result.push_str("%26"),
            '\'' => result.push_str("%27"),
            '(' => result.push_str("%28"),
            ')' => result.push_str("%29"),
            '*' => result.push_str("%2A"),
            '+' => result.push_str("%2B"),
            ',' => result.push_str("%2C"),
            '-' => result.push_str("%2D"),
            '.' => result.push_str("%2E"),
            '/' => result.push_str("%2F"),
            ';' => result.push_str("%3B"),
            '<' => result.push_str("%3C"),
            '=' => result.push_str("%3D"),
            '>' => result.push_str("%3E"),
            '?' => result.push_str("%3F"),
            '@' => result.push_str("%40"),
            '[' => result.push_str("%5B"),
            '\\' => result.push_str("%5C"),
            ']' => result.push_str("%5D"),
            '^' => result.push_str("%5E"),
            '_' => result.push_str("%5F"),
            '`' => result.push_str("%60"),
            '{' => result.push_str("%7B"),
            '|' => result.push_str("%7C"),
            '}' => result.push_str("%7D"),
            '~' => result.push_str("%7E"),
            _ => result.push(c),
        }
    }

    result
}
