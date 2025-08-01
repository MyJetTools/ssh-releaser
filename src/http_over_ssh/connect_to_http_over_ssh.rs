use std::sync::Arc;

use bytes::Bytes;
use flurl::my_ssh::{SshCredentials, SshSession};
use http_body_util::Full;
use hyper::{client::conn::http1::SendRequest, Uri};
use hyper_util::rt::TokioIo;

use crate::execution::ExecuteLogsContainer;

use super::HTTP_CLIENT_TIMEOUT;

const BUFFER_SIZE: usize = 1024 * 512;

pub async fn connect_to_http_over_ssh(
    ssh_credentials: &Arc<SshCredentials>,
    remote_ui: &Uri,
    logs: &Arc<ExecuteLogsContainer>,
) -> (Arc<SshSession>, SendRequest<Full<Bytes>>) {
    let ssh_session = Arc::new(SshSession::new(ssh_credentials.clone()));

    let remote_host = remote_ui.host().unwrap().to_string();
    let port = if let Some(port) = remote_ui.port() {
        port.as_u16()
    } else {
        80
    };

    logs.write_log(format!(
        "Connecting to ssh:{}->{}:{}",
        ssh_session.get_ssh_credentials().get_host_port_as_string(),
        remote_host,
        port
    ))
    .await;

    let ssh_channel = ssh_session
        .connect_to_remote_host(remote_host.as_str(), port, HTTP_CLIENT_TIMEOUT)
        .await
        .unwrap();

    logs.write_log(format!(
        "Connected to ssh:{}->{}:{}",
        ssh_session.get_ssh_credentials().get_host_port_as_string(),
        remote_host,
        port
    ))
    .await;

    let buf_writer = tokio::io::BufWriter::with_capacity(
        BUFFER_SIZE,
        tokio::io::BufReader::with_capacity(BUFFER_SIZE, ssh_channel),
    );

    let io = TokioIo::new(buf_writer);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();

    let proxy_pass_uri = format!("http://{}:{}", remote_host, port);

    tokio::task::spawn(async move {
        if let Err(err) = conn.with_upgrades().await {
            println!("Http Connection to {} is failed: {:?}", proxy_pass_uri, err);
        }

        //Here
    });

    sender.ready().await.unwrap();

    (ssh_session, sender)
}
