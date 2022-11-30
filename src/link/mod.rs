pub mod api;

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AccessMode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "recurrent")]
    Recurrent,
}

impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessMode::Single => write!(f, "single"),
            AccessMode::Recurrent => write!(f, "recurrent"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinkStatus {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    #[serde(rename = "token_required")]
    TokenRequired,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RefreshRate {
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "24h")]
    TwentyFourHours,
    #[serde(rename = "7d")]
    SevenDays,
    #[serde(rename = "30d")]
    ThirtyDays,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkFilters {
    pub access_mode: Option<AccessMode>,
    pub status: Option<LinkStatus>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkBase {
    pub external_id: String,
    pub institution: String,
    pub username: String,
    pub password: String,
    pub access_mode: AccessMode,
}

/// Whenever a user connects to their institution using the Belvo API, we create a Link.
///
/// A Link is a set of credentials, for example the username and password, that is associated with
/// the user.
///
/// You will always need to first register a Link before being able to access information
/// specific to that end user.
#[derive(Deserialize, Debug, Clone)]
pub struct LinkDetail {
    pub id: String,
    pub institution: String,
    pub access_mode: AccessMode,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub last_accessed_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub created_at: DateTime<Utc>,
    pub external_id: String,
    pub institution_user_id: String,
    pub status: LinkStatus,
    pub created_by: String,
    pub refresh_rate: Option<RefreshRate>,
}

impl fmt::Display for LinkDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
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

#[derive(Deserialize, Debug)]
pub struct LinkListResult {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<LinkDetail>,
}
