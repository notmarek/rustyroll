use reqwest::header;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct CrunchyrollClient {
    pub base_url: String,
    pub api_key: String,
    pub user: User,
    pub client: reqwest::Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub country: Option<String>,
    pub expires_in: Option<u32>,
}

impl CrunchyrollClient {
    pub async fn setup(api_key: String, ua: &str, base_url: String, username: &str, password: &str) -> CrunchyrollClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_str(ua).unwrap());
        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        let mut params = HashMap::new();
        params.insert("username", username);
        params.insert("password", password);
        params.insert("grant_type", "password");
        params.insert("scope", "offline_access");
        let user = client
            .post(&format!("{}/auth/v1/token", base_url))
            .header("Authorization", &format!("Basic {}", api_key))
            .form(&params)
            .send()
            .await
            .unwrap()
            .json::<User>()
            .await
            .unwrap();
        CrunchyrollClient {
            api_key: api_key,
            base_url: base_url,
            user: user,
            client: client,
        }
    }
}
