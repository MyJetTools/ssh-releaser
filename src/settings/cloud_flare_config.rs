use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFlareConfig {
    pub id: String,
    pub domain_zone_id: String,
    pub api_key: String,
}
