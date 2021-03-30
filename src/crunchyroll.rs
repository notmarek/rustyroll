use reqwest::header;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct CrunchyrollClient {
    pub base_url: String,
    pub api_key: String,
    pub user: Option<User>,
    pub client: reqwest::Client,
    pub cms: Option<CMSwrapper>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CMS {
    pub bucket: Option<String>,
    pub policy: Option<String>,
    pub signature: Option<String>,
    pub key_pair_id: Option<String>,
    pub expires: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CMSwrapper {
    pub cms: Option<CMS>,
    pub service_available: Option<bool>,
}

impl CrunchyrollClient {
    pub async fn setup(api_key: String, ua: &str, base_url: String, username: &str, password: &str) -> CrunchyrollClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_str(ua).unwrap());
        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        let mut future_self = CrunchyrollClient {
            api_key: api_key,
            base_url: base_url,
            user: None,
            client: client,
            cms: None,
        };
        future_self.load_user(username, password).await;
        future_self.load_cms_info().await;
        future_self
    }

    pub async fn refresh(&mut self) {
        let mut params = HashMap::new();
        params.insert("refresh_token", self.user.as_ref().unwrap().refresh_token.as_ref().unwrap().as_str());
        params.insert("grant_type", "refresh_token");
        params.insert("scope", "offline_access");
        self.user = Some(
            self.client
                .post(&format!("{}/auth/v1/token", self.base_url))
                .header("Authorization", &format!("Basic {}", self.api_key))
                .form(&params)
                .send()
                .await
                .unwrap()
                .json::<User>()
                .await
                .unwrap(),
        );
    }
    async fn load_user(&mut self, username: &str, password: &str) {
        let mut params = HashMap::new();
        params.insert("username", username);
        params.insert("password", password);
        params.insert("grant_type", "password");
        params.insert("scope", "offline_access");
        self.user = Some(
            self.client
                .post(&format!("{}/auth/v1/token", self.base_url))
                .header("Authorization", &format!("Basic {}", self.api_key))
                .form(&params)
                .send()
                .await
                .unwrap()
                .json::<User>()
                .await
                .unwrap(),
        );
    }
    async fn load_cms_info(&mut self) {
        self.cms = Some(
            self.client
                .get(&format!("{}/index/v2", self.base_url))
                .header(
                    "Authorization",
                    &format!("Bearer {}", self.user.as_ref().unwrap().access_token.as_ref().unwrap()),
                )
                .send()
                .await
                .unwrap()
                .json::<CMSwrapper>()
                .await
                .unwrap(),
        );
    }
}
