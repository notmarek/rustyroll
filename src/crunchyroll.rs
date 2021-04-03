use crate::models::*;
use reqwest::header;
use std::collections::HashMap;

pub struct CrunchyrollClient {
    pub base_url: String,
    pub api_key: String,
    pub user: Option<User>,
    pub client: reqwest::Client,
    pub cms: Option<CMSwrapper>,
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
        if future_self.user.as_ref().unwrap().access_token.is_none() {
            future_self.user = None;
        } else {
            future_self.load_cms_info().await;
        }
        future_self
    }

    pub async fn setup_with_proxy(api_key: String, ua: &str, base_url: String, username: &str, password: &str, proxy: reqwest::Proxy) -> CrunchyrollClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_str(ua).unwrap());
        let client = reqwest::Client::builder().default_headers(headers).proxy(proxy).build().unwrap();
        let mut future_self = CrunchyrollClient {
            api_key: api_key,
            base_url: base_url,
            user: None,
            client: client,
            cms: None,
        };
        future_self.load_user(username, password).await;
        if future_self.user.as_ref().unwrap().access_token.is_none() {
            future_self.user = None;
        } else {
            future_self.load_cms_info().await;
        }
        future_self
    }

    pub async fn setup_anon(api_key: String, ua: &str, base_url: String) -> CrunchyrollClient {
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
        println!("Login failed trying anonymous login.");
        future_self.anonymous_login().await;
        future_self.load_cms_info().await;
        future_self
    }

    pub async fn close(&mut self) {
        self.revoke_refresh_token().await;
        self.user = None;
        self.cms = None;
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

    async fn revoke_refresh_token(&self) {
        let mut params = HashMap::new();
        params.insert("token", self.user.as_ref().unwrap().refresh_token.as_ref().unwrap());
        self.client
            .post(&format!("{}/auth/v1/revoke", self.base_url))
            .header("Authorization", &format!("Basic {}", self.api_key))
            .form(&params)
            .send()
            .await
            .unwrap();
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

    async fn anonymous_login(&mut self) {
        let mut params = HashMap::new();
        params.insert("grant_type", "client_id");
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
    pub async fn get_me(&self) -> Me {
        self.client
            .get(&format!("{}/accounts/v1/me", self.base_url))
            .header(
                "Authorization",
                &format!("Bearer {}", self.user.as_ref().unwrap().access_token.as_ref().unwrap()),
            )
            .send()
            .await
            .unwrap()
            .json::<Me>()
            .await
            .unwrap()
    }
    pub async fn get_user_benefits(&self) -> Wrapper<Benefit> {
        self.client
            .get(&format!(
                "{}/subs/v1/subscriptions/{}/benefits",
                self.base_url,
                self.get_me().await.external_id.unwrap()
            ))
            .header(
                "Authorization",
                &format!("Bearer {}", self.user.as_ref().unwrap().access_token.as_ref().unwrap()),
            )
            .send()
            .await
            .unwrap()
            .json::<Wrapper<Benefit>>()
            .await
            .unwrap()
    }
    pub async fn search(&self, query: &str) -> Wrapper<Wrapper<SearchItem>> {
        self.client
            .get(&format!("{}/content/v1/search?q={}&type=series&locale=en-US", self.base_url, query))
            .header(
                "Authorization",
                &format!("Bearer {}", self.user.as_ref().unwrap().access_token.as_ref().unwrap()),
            )
            .send()
            .await
            .unwrap()
            .json::<Wrapper<Wrapper<SearchItem>>>()
            .await
            .unwrap()
    }

    pub async fn get_episode(&self, episode_id: &str) -> Episode {
        let cms = self.cms.as_ref().unwrap().cms.as_ref().unwrap();
        self.client
            .get(&format!(
                "{}/cms/v2{}/episodes/{}?Policy={}&Signature={}&Key-Pair-Id={}&locale=en-US",
                self.base_url,
                cms.bucket.as_ref().unwrap(),
                episode_id,
                cms.policy.as_ref().unwrap(),
                cms.signature.as_ref().unwrap(),
                cms.key_pair_id.as_ref().unwrap()
            ))
            .send()
            .await
            .unwrap()
            .json::<Episode>()
            .await
            .unwrap()
    }

    pub async fn get_episodes(&self, season_id: &str) -> Wrapper<Episode> {
        let cms = self.cms.as_ref().unwrap().cms.as_ref().unwrap();
        self.client
            .get(&format!(
                "{}/cms/v2{}/episodes?season_id={}&Policy={}&Signature={}&Key-Pair-Id={}&locale=en-US",
                self.base_url,
                cms.bucket.as_ref().unwrap(),
                season_id,
                cms.policy.as_ref().unwrap(),
                cms.signature.as_ref().unwrap(),
                cms.key_pair_id.as_ref().unwrap()
            ))
            .send()
            .await
            .unwrap()
            .json::<Wrapper<Episode>>()
            .await
            .unwrap()
    }

    pub async fn get_seasons(&self, series_id: &str) -> Wrapper<Season> {
        let cms = self.cms.as_ref().unwrap().cms.as_ref().unwrap();
        self.client
            .get(&format!(
                "{}/cms/v2{}/seasons?series_id={}&Policy={}&Signature={}&Key-Pair-Id={}&locale=en-US",
                self.base_url,
                cms.bucket.as_ref().unwrap(),
                series_id,
                cms.policy.as_ref().unwrap(),
                cms.signature.as_ref().unwrap(),
                cms.key_pair_id.as_ref().unwrap()
            ))
            .send()
            .await
            .unwrap()
            .json::<Wrapper<Season>>()
            .await
            .unwrap()
    }

    pub async fn get_series(&self, series_id: &str) -> Series {
        let cms = self.cms.as_ref().unwrap().cms.as_ref().unwrap();
        self.client
            .get(&format!(
                "{}/cms/v2{}/series/{}?Policy={}&Signature={}&Key-Pair-Id={}&locale=en-US",
                self.base_url,
                cms.bucket.as_ref().unwrap(),
                series_id,
                cms.policy.as_ref().unwrap(),
                cms.signature.as_ref().unwrap(),
                cms.key_pair_id.as_ref().unwrap()
            ))
            .send()
            .await
            .unwrap()
            .json::<Series>()
            .await
            .unwrap()
    }
    pub async fn get_video_streams_by_media_id(&self, media_id: &str) -> Video {
        let cms = self.cms.as_ref().unwrap().cms.as_ref().unwrap();
        self.client
            .get(&format!(
                "{}/cms/v2{}/videos/{}/streams?Policy={}&Signature={}&Key-Pair-Id={}&locale=en-US",
                self.base_url,
                cms.bucket.as_ref().unwrap(),
                media_id,
                cms.policy.as_ref().unwrap(),
                cms.signature.as_ref().unwrap(),
                cms.key_pair_id.as_ref().unwrap()
            ))
            .header(
                "Authorization",
                &format!("Bearer {}", self.user.as_ref().unwrap().access_token.as_ref().unwrap()),
            )
            .send()
            .await
            .unwrap()
            .json::<Video>()
            .await
            .unwrap()
    }
    pub async fn get_video_streams_by_link(&self, link: &str) -> Video {
        let cms = self.cms.as_ref().unwrap().cms.as_ref().unwrap();
        self.client
            .get(&format!(
                "{}{}?Policy={}&Signature={}&Key-Pair-Id={}&locale=en-US",
                self.base_url,
                link,
                cms.policy.as_ref().unwrap(),
                cms.signature.as_ref().unwrap(),
                cms.key_pair_id.as_ref().unwrap()
            ))
            .header(
                "Authorization",
                &format!("Bearer {}", self.user.as_ref().unwrap().access_token.as_ref().unwrap()),
            )
            .send()
            .await
            .unwrap()
            .json::<Video>()
            .await
            .unwrap()
    }
}
