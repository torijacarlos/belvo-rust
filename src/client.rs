use core::fmt;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

///
#[derive(Deserialize, Debug)]
pub enum Environment {
    Sandbox,
    Development,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Sandbox => write!(f, "sandbox"),
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production"),
        }
    }
}

/// Set of API Keys used to consume Belvo's API
#[derive(Deserialize, Clone)]
pub struct BelvoKey {
    pub secret_id: String,
    pub secret_pwd: String,
}

impl BelvoKey {
    ///
    pub fn to_base64(&self) -> String {
        let secrets = format!("{}:{}", self.secret_id, self.secret_pwd);
        base64::encode(secrets)
    }
}

///
pub struct BelvoClient {
    key: BelvoKey,
    env: Environment,
}

impl BelvoClient {
    ///
    pub fn new(key: BelvoKey, env: Environment) -> Self {
        BelvoClient { key, env }
    }

    ///
    pub fn base_url(&self) -> String {
        format!("https://{}.belvo.com", self.env)
    }

    ///
    pub fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Basic {}", self.key.to_base64()))
                .expect("Invalid authorization header content"),
        );
        headers.insert(
            "accept",
            HeaderValue::from_str("application/json").expect("Invalid accept header"),
        );
        headers.insert(
            "content-type",
            HeaderValue::from_str("application/json").expect("Invalid Content Type"),
        );
        headers
    }
}
