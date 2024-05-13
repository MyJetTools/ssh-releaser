use std::sync::Arc;

use crate::{environment::EnvContext, settings::WriteCloudFlareDomainARecordModel};

use super::{ExecuteCommandError, ExecuteLogsContainer};

pub async fn execute_cloud_flare_write_domain(
    env_settings: &EnvContext,
    model: WriteCloudFlareDomainARecordModel,
    logs: &Arc<ExecuteLogsContainer>,
) -> Result<(), ExecuteCommandError> {
    let config = env_settings.find_cloud_flare_config(&model.domain);

    if config.is_none() {
        return Err(format!(
            "CloudFlare config not found to write domain a-record: {}",
            model.domain
        )
        .into());
    }

    let config = config.unwrap();

    logs.write_log(format!(
        "Writing domain a-record {} -> {}",
        model.ip, model.domain
    ))
    .await;

    cloudflare_sdk::dns_records::create(
        &config.domain_zone_id,
        &config.api_key,
        model.domain.to_string(),
        model.is_proxy,
        model.ip.to_string(),
    )
    .await?;

    logs.write_log(format!(
        "Written domain a-record {} -> {}",
        model.ip, model.domain
    ))
    .await;

    Ok(())
}
