use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Deserialize, Serialize)]
pub struct RequestTokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub entityid: i64,
    pub userid: i64,
    #[serde(rename = "entityType")]
    pub entity_type: EntityType,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(i16)]
pub enum EntityType {
    User = 0,
    Group = 1,
    CreditAccount = 2,
}
