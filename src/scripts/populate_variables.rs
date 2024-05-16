use std::sync::Arc;

use rust_extensions::StrOrString;

use crate::{environment::EnvContext, execution::*};

use super::PopulateVariablesProcessing;

pub const PLACEHOLDER_OPEN_TOKEN: &str = "${";
pub const PLACEHOLDER_CLOSE_TOKEN: &str = "}";

pub async fn populate_variables<'s>(
    settings: &'s EnvContext,
    script_env: Option<&impl ScriptEnvironment>,
    src: &'s str,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<StrOrString<'s>, ExecuteCommandError> {
    let index = src.find(PLACEHOLDER_OPEN_TOKEN);

    if index.is_none() {
        return Ok(src.into());
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
                    settings,
                    script_env,
                    placeholder_to_process,
                    populate_placeholders_after_reading_from_file,
                    logs,
                )
                .await?;

                if processing.has_url_encoded() {
                    let url_encoded = super::convert_url_encoded(content.as_str());
                    result.push_str(url_encoded.as_str());
                } else {
                    result.push_str(content.as_str());
                }
            }
        }
    }

    Ok(result.into())
}

async fn get_placeholder_content<'s>(
    settings: &'s EnvContext,
    script_env: Option<&'s impl ScriptEnvironment>,
    placeholder: &str,
    populate_placeholders_after_reading_from_file: bool,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<StrOrString<'s>, ExecuteCommandError> {
    if placeholder.starts_with("/") || placeholder.starts_with("~") || placeholder.starts_with(".")
    {
        let (mut content, _) =
            crate::scripts::load_file(settings, script_env, placeholder, logs).await?;
        if populate_placeholders_after_reading_from_file {
            content = super::populate_variables_after_loading_from_file(
                settings,
                script_env,
                content,
                PLACEHOLDER_OPEN_TOKEN,
            )?;
        }

        return Ok(content.into());
    }

    if placeholder.starts_with("$") {
        let mut result = String::new();
        result.push_str("${");
        result.push_str(placeholder[1..].as_ref());
        result.push('}');

        return Ok(result.into());
    }

    Ok(settings.get_env_variable(script_env, placeholder)?)
}
