use std::sync::Arc;

use flurl::FlUrl;

use crate::{
    app::AppContext,
    caching::HttpResponseCache,
    execution::{ExecuteCommandError, ExecuteLogsContainer},
    file_path::FilePathRef,
};

#[derive(Clone)]
pub struct FileName(String);

impl FileName {
    pub fn new(src: String) -> Self {
        Self(src)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn get_file_path<'s>(&'s self) -> FilePathRef<'s> {
        let src = self.0.as_bytes();

        let mut index = src.len();

        while index > 0 {
            if let Some(b'/') = src.get(index - 1) {
                return FilePathRef::new(std::str::from_utf8(&src[0..index]).unwrap());
            }
            index -= 1;
        }

        FilePathRef::new(self.0.as_str())
    }

    pub async fn load_content_as_string(
        &self,
        app: &AppContext,
        logs: &Arc<ExecuteLogsContainer>,
    ) -> Result<String, ExecuteCommandError> {
        let content = self.load_content(app, logs).await?;
        Ok(String::from_utf8(content)?)
    }

    pub async fn load_content(
        &self,
        app: &AppContext,
        logs: &Arc<ExecuteLogsContainer>,
    ) -> Result<Vec<u8>, ExecuteCommandError> {
        if !self.as_str().starts_with("http") {
            logs.write_log(format!("Loading content from file: '{}'", self.as_str()))
                .await;
            let content = tokio::fs::read(self.as_str()).await;

            match content {
                Ok(content) => {
                    return Ok(content);
                }
                Err(e) => {
                    return Err(format!(
                        "Error loading content from local file: '{}'. Err: {:?}",
                        self.as_str(),
                        e
                    )
                    .into());
                }
            }
        }

        let url = self.as_str().to_lowercase();
        let mut cache_access = app.cached_http_get_requests.lock().await;

        let response = cache_access.get_cached_response(url.as_str());

        if let Some(response) = response {
            return Ok(response);
        }

        logs.write_log(format!(
            "Loading content from remote resource: '{}'",
            self.as_str()
        ))
        .await;

        let fl_url = FlUrl::new(url.as_str())
            .do_not_reuse_connection()
            .get()
            .await
            .unwrap();
        let result = fl_url.receive_body().await?;

        let cached_response = HttpResponseCache::new(result.clone());

        cache_access.set_cached_response(url, cached_response);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_name::FileName;

    #[test]
    fn test_get_file_path() {
        let src = "/test/test2/my_file.yaml".to_string();

        let file_name = FileName::new(src);

        assert_eq!("/test/test2/", file_name.get_file_path().as_str());
    }

    #[test]
    fn test_get_file_path_root() {
        let src = "/my_file.yaml".to_string();

        let file_name = FileName::new(src);

        assert_eq!("/", file_name.get_file_path().as_str());
    }
}
