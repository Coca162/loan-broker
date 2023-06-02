use std::sync::Arc;

use dashmap::DashSet;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::OnceLock;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SharedData {
    pub pool: PgPool,
    pub oauth2_states: Arc<DashSet<u32>>,
    pub env_config: Arc<EnvConfig>,
    pub reqwest: reqwest::Client,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EnvConfig {
    pub oauth2_client_id: u64,
    pub oauth2_client_secret: Uuid,
    pub website_url: String,
    #[serde(skip)]
    oauth2_redirect_uri: OnceLock<String>,
}

impl EnvConfig {
    pub fn oauth2_redirect_uri(&self) -> &String {
        self.oauth2_redirect_uri
            .get_or_init(|| format!("{}/api/callback", self.website_url))
    }
}
