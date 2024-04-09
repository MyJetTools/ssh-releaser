use std::{collections::HashMap, sync::Arc};

use http_body_util::Full;
use hyper::{body::Bytes, client::conn::http1::SendRequest, Method, Request, Uri};
use my_ssh::{SshCredentials, SshSession};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

use super::HttpClientError;
pub const HTTP_CLIENT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

pub struct Http1Client {
    pub connected: DateTimeAsMicroseconds,
    pub send_request: Mutex<Option<SendRequest<Full<Bytes>>>>,
    pub ssh_session: Arc<SshSession>,
}

impl Http1Client {
    pub async fn connect(
        ssh_credentials: &Arc<SshCredentials>,
        remote_host: &Uri,
    ) -> Result<Self, HttpClientError> {
        let (ssh_session, send_request) =
            Self::connect_to_http_over_ssh(ssh_credentials, remote_host).await;

        let result = Self {
            send_request: Mutex::new(Some(send_request)),
            connected: DateTimeAsMicroseconds::now(),
            ssh_session,
        };

        Ok(result)
    }

    async fn connect_to_http_over_ssh(
        credentials: &Arc<SshCredentials>,
        remote_host: &Uri,
    ) -> (Arc<SshSession>, SendRequest<Full<Bytes>>) {
        let result = super::connect_to_http_over_ssh(credentials, remote_host).await;

        result
    }

    pub async fn post(
        &self,
        uri: Uri,
        body: Vec<u8>,
        headers: &Option<HashMap<String, String>>,
    ) -> Result<u16, HttpClientError> {
        let body = http_body_util::Full::new(body.into());
        let mut request = Request::builder().uri(uri).method(Method::POST);

        if let Some(headers) = headers {
            for (k, v) in headers {
                request = request.header(k, v);
            }
        }

        let request = request.body(body).unwrap();

        let mut send_request = self.send_request.lock().await;

        let result = send_request
            .as_mut()
            .unwrap()
            .send_request(request)
            .await
            .unwrap();
        Ok(result.status().as_u16())
    }
}
