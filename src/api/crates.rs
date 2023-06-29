use gloo_net::http::Request;
use gloo_net::Error;
use serde::Deserialize;
use serde_json::json;

use super::{APP_HOST, AUTHORIZATION, CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE};

#[derive(Deserialize, Clone, PartialEq)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: String,
}

pub async fn api_crates(token: &String) -> Result<Vec<Crate>, Error> {
    let response = Request::get(&format!("{}/crates", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Vec<Crate>>().await
}

pub async fn api_crate_create(
    token: &String,
    name: String,
    code: String,
    rustacean_id: i32,
    version: String,
    description: String,
) -> Result<Crate, Error> {
    let response = Request::post(&format!("{}/crates", APP_HOST))
        .header(AUTHORIZATION, &format!("Bearer {}", token))
        .header(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
        .json(&json!({
            "name": name,
            "code": code,
            "rustacean_id": rustacean_id,
            "version": version,
            "description": description
        }))?
        .send()
        .await?;

    response.json::<Crate>().await
}

pub async fn api_crate_update(
    token: &String,
    id: i32,
    name: String,
    code: String,
) -> Result<Crate, Error> {
    let response = Request::put(&format!("{}/crates/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .header(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
        .json(&json!({
            "name": name,
            "email": code
        }))?
        .send()
        .await?;

    response.json::<Crate>().await
}
