use std::sync::Arc;

use settings::GlobalSettingsModel;

mod app;

mod environment;

mod execute_from_template;

mod caching;
mod execution;
mod file_name;
mod file_path;
mod http_over_ssh;
mod http_server;
mod scripts;
mod settings;

#[tokio::main]
async fn main() {
    flurl::my_tls::install_default_crypto_providers();

    let my_settings = GlobalSettingsModel::read_from_file("~/.ssh-releaser".to_string())
        .await
        .unwrap();

    let app = Arc::new(app::AppContext::new(my_settings).await);

    crate::http_server::setup_server(&app);

    app.app_states.wait_until_shutdown().await;
}
