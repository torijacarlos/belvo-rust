use std::error::Error;

use reqwest::{Client, StatusCode, Url};

use crate::client::BelvoClient;
use crate::link::{LinkBase, LinkDetail, LinkFilters, LinkListResult};

pub async fn register(
    base: &LinkBase,
    belvo_client: &BelvoClient,
) -> Result<LinkDetail, Box<dyn Error>> {
    let body = serde_json::to_string(&base)?;
    let url = Url::parse(&format!("{}/api/links", belvo_client.base_url())[..])?;
    let client = Client::new()
        .post(url)
        .headers(belvo_client.headers())
        .body(body);
    let response = client.send().await?;
    let result = response.text().await?;

    let link: LinkDetail = serde_json::from_str(&result)?;
    Ok(link)
}

pub async fn list(
    filters: &LinkFilters,
    belvo_client: &BelvoClient,
) -> Result<LinkListResult, Box<dyn Error>> {
    let url = Url::parse(&format!("{}/api/links", belvo_client.base_url())[..])?;
    let client = Client::new()
        .get(url)
        .headers(belvo_client.headers())
        .query(&filters);
    let response = client.send().await?;
    let result = response.text().await?;

    let link_results: LinkListResult = serde_json::from_str(&result)?;

    Ok(link_results)
}

pub async fn delete(id: &String, belvo_client: &BelvoClient) -> Result<StatusCode, Box<dyn Error>> {
    let url = Url::parse(&format!("{}/api/links/{}", belvo_client.base_url(), id)[..])?;

    let response = Client::new()
        .delete(url)
        .headers(belvo_client.headers())
        .send()
        .await?;

    Ok(response.status())
}
