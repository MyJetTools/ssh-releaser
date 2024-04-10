use crate::app::AppContext;

use super::{PLACEHOLDER_CLOSE_TOKEN, PLACEHOLDER_OPEN_TOKEN};

pub async fn populate_for_post_request(app: &AppContext, content: String) -> String {
    let index = content.find(PLACEHOLDER_OPEN_TOKEN);

    if index.is_none() {
        return content;
    }
    let mut result = String::new();

    for token in rust_extensions::placeholders::PlaceholdersIterator::new(
        content.as_str(),
        PLACEHOLDER_OPEN_TOKEN,
        PLACEHOLDER_CLOSE_TOKEN,
    ) {
        match token {
            rust_extensions::placeholders::ContentToken::Text(text) => result.push_str(text),
            rust_extensions::placeholders::ContentToken::Placeholder(placeholder) => {
                let file_content = super::load_file(app, placeholder, true).await;
                let file_content = convert_url_encoded(file_content);
                result.push_str(file_content.as_str());
            }
        }
    }

    result.into()
}
