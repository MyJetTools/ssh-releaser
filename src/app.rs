use std::{collections::HashMap, sync::Arc};

use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::{
    caching::HttpGetRequestsCache, execution::ExecuteLogsContainer, settings::GlobalSettingsModel,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub global_settings: GlobalSettingsModel,
    pub app_states: Arc<AppStates>,
    pub containers: Mutex<HashMap<String, Arc<ExecuteLogsContainer>>>,
    pub cached_http_get_requests: Mutex<HttpGetRequestsCache>,
}

impl AppContext {
    pub async fn new(global_settings: GlobalSettingsModel) -> AppContext {
        AppContext {
            global_settings,
            app_states: Arc::new(AppStates::create_initialized()),
            containers: Mutex::new(HashMap::new()),
            cached_http_get_requests: Mutex::new(HttpGetRequestsCache::new()),
        }
    }

    pub async fn add_container(&self, container: Arc<ExecuteLogsContainer>) {
        let mut containers = self.containers.lock().await;
        containers.insert(container.id.clone(), container);
    }

    pub async fn get_container(&self, id: &str) -> Option<Arc<ExecuteLogsContainer>> {
        let containers = self.containers.lock().await;
        containers.get(id).cloned()
    }
}
