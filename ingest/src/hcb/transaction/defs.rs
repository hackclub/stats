use crate::utils::{deserialize_datetime, deserialize_optional_datetime};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct CanonicalEventMapping {
    id: i64,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    canonical_transaction_id: Option<i64>,
    event_id: Option<i64>,
    subledger_id: Option<i64>,
    user_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct CanonicalHashedMapping {
    id: i64,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    canonical_transaction_id: Option<i64>,
    hashed_transaction_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct CanonicalPendingDeclinedMapping {
    id: i64,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    canonical_pending_transaction_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct CanonicalPendingEventMapping {
    id: i64,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    canonical_pending_transaction_id: Option<i64>,
    event_id: Option<i64>,
    subledger_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct CanonicalPendingSettledMapping {
    id: i64,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    canonical_pending_transaction_id: Option<i64>,
    canonical_transaction_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct CanonicalPendingTransaction {
    id: i64,
    amount_cents: i64,
    custom_memo: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    date: Option<DateTime<Utc>>,
    fee_waived: bool,
    fronted: bool,
    hcb_code: Option<String>,
    memo: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    ach_payment_id: Option<i64>,
    check_deposit_id: Option<i64>,
    grant_id: Option<i64>,
    increase_check_id: Option<i64>,
    raw_pending_bank_fee_transaction_id: Option<i64>,
    raw_pending_donation_transaction_id: Option<i64>,
    raw_pending_incoming_disbursement_transaction_id: Option<i64>,
    raw_pending_invoice_transaction_id: Option<i64>,
    raw_pending_outgoing_ach_transaction_id: Option<i64>,
    raw_pending_outgoing_check_transaction_id: Option<i64>,
    raw_pending_outgoing_disbursement_transaction_id: Option<i64>,
    raw_pending_partner_donation_transaction_id: Option<i64>,
    raw_pending_stripe_transaction_id: Option<i64>,
}

// #[derive(Deserialize, Serialize, Debug, fake::Dummy)]
// pub struct CanonicalTransaction {
//     id: i64,
//     amount_cents: i64,
//     custom_memo: Option<String>,
//     #[serde(deserialize_with = "deserialize_optional_datetime")]
//     date: Option<DateTime<Utc>>,
//     friendly_memo: Option<String>,
//     hcb_code: Option<String>,
//     memo: Option<String>,
//     transaction_source_type: String,
//     #[serde(deserialize_with = "deserialize_optional_datetime")]
//     created_at: Option<DateTime<Utc>>,
//     #[serde(deserialize_with = "deserialize_optional_datetime")]
//     updated_at: Option<DateTime<Utc>>,
//     transaction_source_id: Option<i64>,
// }

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct CanonicalTransaction {
    amount_cents: i64,
    #[serde(deserialize_with = "deserialize_datetime")]
    created_at: DateTime<Utc>,
    custom_memo: Option<String>, // Can be null
    date: String,                // ISO 8601 format
    friendly_memo: String,
    hcb_code: String,
    id: i64,
    memo: String,
    transaction_source_id: i64,
    transaction_source_type: String,
    #[serde(deserialize_with = "deserialize_datetime")]
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, fake::Dummy)]
pub struct HashedTransaction {
    id: i64,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    date: Option<DateTime<Utc>>,
    primary_hash: Option<String>,
    primary_hash_input: Option<String>,
    secondary_hash: Option<String>,
    unique_bank_identifier: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    created_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    updated_at: Option<DateTime<Utc>>,
    duplicate_of_hashed_transaction_id: Option<i64>,
    raw_csv_transaction_id: Option<i64>,
    raw_emburse_transaction_id: Option<i64>,
    raw_increase_transaction_id: Option<i64>,
    raw_plaid_transaction_id: Option<i64>,
    raw_stripe_transaction_id: Option<i64>,
}
