use crate::utils::deserialize_optional_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct User {
    access_level: String,
    // #[serde(deserialize_with = "deserialize_optional_datetime")]
    // admin_at: Option<DateTime<Utc>>, // Can be null
    // #[serde(deserialize_with = "deserialize_optional_datetime")]
    // birthday: Option<DateTime<Utc>>, // Can be null
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    email: String,
    full_name: Option<String>, // Can be null
    id: i64,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    locked_at: Option<DateTime<Utc>>, // Can be null
    phone_number: String,
    phone_number_verified: bool,
    preferred_name: Option<String>, // Can be null
    pretend_is_not_admin: bool,
    receipt_report_option: String,
    running_balance_enabled: bool,
    seasonal_themes_enabled: bool,
    session_duration_seconds: i64,
    sessions_reported: bool,
    slug: String,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    use_sms_auth: bool,
    webauthn_id: Option<String>, // Can be null
}
impl crate::hcb::HcbModel for User {}
