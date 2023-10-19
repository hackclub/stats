use crate::utils::deserialize_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    created_at: String,
    // deleted_at ∈ {Null}
    device_info: Option<String>, // Can be null
    #[serde(deserialize_with = "deserialize_datetime")]
    expiration_at: DateTime<Utc>,
    fingerprint: Option<String>, // Can be null
    id: i64,
    impersonated_by_id: Option<i64>, // Can be null
    ip: Option<String>,              // Can be null
    latitude: Option<String>,        // Can be null
    longitude: Option<String>,       // Can be null
    os_info: Option<String>,         // Can be null
    // peacefully_expired ∈ {Null}
    session_token_bidx: String,
    timezone: Option<String>, // Can be null
    #[serde(deserialize_with = "deserialize_datetime")]
    updated_at: DateTime<Utc>,
    user_id: i64,
    webauthn_credential_id: Option<i64>, // Can be null
}
impl crate::hcb::HcbModel for Session {}
