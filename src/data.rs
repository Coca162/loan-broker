use std::sync::Arc;

use dashmap::DashSet;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SharedData {
    pub pool: PgPool,
    pub oauth2_states: DashSet<u32>,
    pub env_config: Arc<EnvConfig>,
    pub reqwest: reqwest::Client
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EnvConfig {
    pub oauth2_client_id: u64,
    pub oauth2_client_secret: Uuid,
    pub website_url: String,
}
