use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result
        .register_get_action(super::env_controller::GetEnvironmentsAction::new(app.clone()).into());

    result.register_get_action(
        super::release_data_controller::GetReleaseDataAction::new(app.clone()).into(),
    );

    result.register_post_action(
        super::release_data_controller::ExecuteAction::new(app.clone()).into(),
    );

    result
}
