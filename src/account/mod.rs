pub mod api;

use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use std::option::Option;

#[derive(Deserialize, Debug, Clone)]
pub enum Currency {
    MXN,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Institution {
    pub name: String,
    #[serde(rename = "type")]
    pub ins_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Balance {
    pub current: f64,
    pub available: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreditData {
    #[serde(deserialize_with = "deserialize_from_str")]
    pub collected_at: DateTime<Utc>,
    pub credit_limit: f64,
    #[serde(skip)]
    pub next_payment_date: DateTime<Utc>,
    #[serde(skip)]
    pub cutting_date: Option<DateTime<Utc>>,
    pub minimum_payment: f64,
    pub monthly_payment: f64,
    pub no_interest_payment: f64,
    #[serde(skip)]
    pub last_payment_date: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub last_period_balance: Option<DateTime<Utc>>,
    pub interest_rate: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub link: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub collected_at: DateTime<Utc>,
    #[serde(skip)]
    pub last_accessed_at: Option<DateTime<Utc>>,
    pub name: String,
    pub internal_identification: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub category: String,
    pub bank_product_id: Option<String>,
    pub public_identification_name: String,
    pub public_identification_value: String,
    pub currency: Currency,
    pub credit_data: Option<CreditData>,
    pub loan_data: Option<String>,
    pub number: String,
    pub balance_type: String,
    pub institution: Institution,
    pub balance: Balance,
}

// You can use this deserializer for any type that implements FromStr
// and the FromStr::Err implements Display
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,      // Required for S::from_str...
    S::Err: Display, // Required for .map_err(de::Error::custom)
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}
