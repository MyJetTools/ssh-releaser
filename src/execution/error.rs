use cloudflare_sdk::CloudFlareError;
use rust_extensions::StrOrString;

#[derive(Debug)]
pub enum ExecuteCommandError {
    JustError(String),
    CloudFlareError(CloudFlareError),
    HttpOverSshClientError(crate::http_over_ssh::HttpClientError),
    SshSessionError(my_ssh::SshSessionError),
}

impl From<my_ssh::SshSessionError> for ExecuteCommandError {
    fn from(error: my_ssh::SshSessionError) -> Self {
        ExecuteCommandError::SshSessionError(error)
    }
}

impl Into<StrOrString<'static>> for ExecuteCommandError {
    fn into(self) -> StrOrString<'static> {
        match self {
            ExecuteCommandError::JustError(error) => error.into(),
            _ => format!("{:?}", self).into(),
        }
    }
}

impl From<crate::http_over_ssh::HttpClientError> for ExecuteCommandError {
    fn from(error: crate::http_over_ssh::HttpClientError) -> Self {
        ExecuteCommandError::HttpOverSshClientError(error)
    }
}

impl From<CloudFlareError> for ExecuteCommandError {
    fn from(error: CloudFlareError) -> Self {
        ExecuteCommandError::CloudFlareError(error)
    }
}

impl Into<ExecuteCommandError> for String {
    fn into(self) -> ExecuteCommandError {
        ExecuteCommandError::JustError(self)
    }
}
