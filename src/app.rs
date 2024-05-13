use std::{collections::HashMap, sync::Arc};

use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::{execution::ExecuteLogsContainer, settings::GlobalSettingsModel};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub global_settings: GlobalSettingsModel,
    pub app_states: Arc<AppStates>,

    pub containers: Mutex<HashMap<String, Arc<ExecuteLogsContainer>>>,
}

impl AppContext {
    pub async fn new(global_settings: GlobalSettingsModel) -> AppContext {
        AppContext {
            global_settings,
            app_states: Arc::new(AppStates::create_initialized()),
            containers: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_container(&self, container: Arc<ExecuteLogsContainer>) {
        let mut containers = self.containers.lock().await;
        containers.insert(container.id.clone(), container);
    }
}
