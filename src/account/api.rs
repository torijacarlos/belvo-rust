use std::error::Error;

use reqwest::{Client, Url};
use serde::Serialize;

use crate::client::BelvoClient;

use super::Account;

pub async fn list(
    link_id: &String,
    save_data: bool,
    belvo_client: &BelvoClient,
) -> Result<Vec<Account>, Box<dyn Error>> {
    #[derive(Serialize)]
    struct AccountRequest {
        link: String,
        save_data: bool,
    }
    let body = serde_json::to_string(&AccountRequest {
        link: link_id.to_string(),
        save_data,
    })?;
    let url = Url::parse(&format!("{}/api/accounts", belvo_client.base_url())[..])?;
    let client = Client::new()
        .post(url)
        .headers(belvo_client.headers())
        .body(body);
    let response = client.send().await?;
    let result = response.text().await?;
    let accounts: Vec<Account> = serde_json::from_str(&result)?;
    Ok(accounts)
}
