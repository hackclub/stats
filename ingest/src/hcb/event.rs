use crate::utils::{deserialize_f64_from_string, deserialize_optional_datetime};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct Event {
    aasm_state: String,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    activated_at: Option<DateTime<Utc>>, // Can be null
    address: Option<String>,             // Can be null
    beta_features_enabled: Option<bool>, // Can be null
    can_front_balance: bool,
    #[serde(rename(serialize = "hcb_category"))]
    category: Option<String>, // Can be null
    club_airtable_id: Option<String>, // Can be null
    country: Option<String>,          // Can be null
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    custom_css_url: Option<String>, // Can be null
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    deleted_at: Option<DateTime<Utc>>, // ∈ {Null}
    demo_mode: bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    demo_mode_request_meeting_at: Option<DateTime<Utc>>, // Can be null
    description: Option<String>,                   // Can be null
    donation_page_enabled: Option<bool>,           // Can be null
    donation_page_message: Option<String>,         // Can be null
    emburse_department_id: Option<String>,         // Can be null
    end: Option<String>,                           // Can be null
    expected_budget: Option<i64>,                  // Can be null
    has_fiscal_sponsorship_document: Option<bool>, // Can be null
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    hidden_at: Option<DateTime<Utc>>, // Can be null
    holiday_features: bool,
    id: i64,
    // increase_account_id: String,
    is_indexable: bool,
    is_public: bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    last_fee_processed_at: Option<DateTime<Utc>>, // Can be null
    name: String,
    omit_stats: bool,
    organization_identifier: String,
    // owner_address: Option<String>,   // Can be null
    // owner_birthdate: Option<String>, // Can be null
    // owner_email: Option<String>,     // Can be null
    // owner_name: Option<String>,      // Can be null
    // owner_phone: Option<String>,     // Can be null
    partner_id: Option<i64>, // Can be null
    pending_transaction_engine_at: String,
    point_of_contact_id: Option<i64>, // Can be null
    public_message: Option<String>,   // Can be null
    redirect_url: Option<String>,     // Can be null
    slug: String,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    sponsorship_fee: f64,
    start: Option<String>, // Can be null
    stripe_card_shipping_type: String,
    transaction_engine_v2_at: String,
    updated_at: String,
    webhook_url: Option<String>, // Can be null
    website: Option<String>,     // Can be null
}
impl crate::hcb::HcbModel for Event {}
