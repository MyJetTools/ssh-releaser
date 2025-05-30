use cloudflare_sdk::CloudFlareError;
use flurl::FlUrlError;
use rust_extensions::StrOrString;

#[derive(Debug)]
pub enum ExecuteCommandError {
    JustError(String),
    CloudFlareError(CloudFlareError),
    SshSessionError(flurl::my_ssh::SshSessionError),
    FlUrlError(FlUrlError),
    IoError(std::io::Error),
}

impl From<std::string::FromUtf8Error> for ExecuteCommandError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ExecuteCommandError::JustError(error.to_string())
    }
}

impl From<std::io::Error> for ExecuteCommandError {
    fn from(error: std::io::Error) -> Self {
        ExecuteCommandError::IoError(error)
    }
}

impl From<FlUrlError> for ExecuteCommandError {
    fn from(error: FlUrlError) -> Self {
        ExecuteCommandError::FlUrlError(error)
    }
}

impl From<flurl::my_ssh::SshSessionError> for ExecuteCommandError {
    fn from(error: flurl::my_ssh::SshSessionError) -> Self {
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
