use crate::{app::AppContext, settings::WriteCloudFlareDomainARecordModel};

pub async fn execute_cloud_flare_write_domain(
    app: &AppContext,
    model: WriteCloudFlareDomainARecordModel,
) {
    let config = app.find_cloud_flare_config(&model.domain);

    if config.is_none() {
        panic!(
            "CloudFlare config not found to write domain a-record: {}",
            model.domain
        );
    }

    let config = config.unwrap();

    cloudflare_sdk::dns_records::create(
        &config.domain_zone_id,
        &config.api_key,
        model.domain,
        model.is_proxy,
        model.ip,
    )
    .await
    .unwrap();
}
