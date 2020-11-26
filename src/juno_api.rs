use serde_json;
use base64::{encode as base64_encode};
use std::{time::Duration};
use ttl_cache::TtlCache;
use lazy_static::*;
use std::sync::Mutex;
use crate::messages::AbstractRequest;

pub struct JunoApi {
    test_mode: bool,
    client_id: String,
    client_secret: String,
    resource_token: Option<String>,
}

impl JunoApi {
    const LIVE_BASE_URL: &'static str       = "https://api.juno.com.br";
    const TEST_BASE_URL: &'static str       = "https://sandbox.boletobancario.com/api-integration";
    const AUTH_LIVE_ENDPOINT: &'static str  = "https://api.juno.com.br/authorization-server/oauth/token";
    const AUTH_TEST_ENDPOINT: &'static str  = "https://sandbox.boletobancario.com/authorization-server/oauth/token";
    
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            test_mode: true,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            resource_token: None,
        }
    }

    pub fn test_mode(mut self, value: bool) -> Self {
        self.test_mode = value;
        self
    }

    pub fn resource_token(mut self, value: String) -> Self {
        self.resource_token = Some(value);
        self
    }

    pub async fn request(
        &self,
        req: impl AbstractRequest,
    ) -> Result<reqwest::Response, reqwest::Error>  {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.append("Authorization", self.get_bearer_authorization().await.parse().unwrap());
        headers.extend(req.headers());

        let url = format!("{}/{}", self.base_url(), req.endpoint());

        let request = reqwest::Client::new()
            .request(
                req.http_method(),
                reqwest::Url::parse(url.as_str()).unwrap(),
            )
            .headers(headers)
            .body(serde_json::to_string(&req.data()).unwrap());

        request.send().await
    }

    fn base_url(&self) -> &str {
        if self.test_mode {Self::TEST_BASE_URL} else {Self::LIVE_BASE_URL}
    }

    fn auth_url(&self) -> &str {
        if self.test_mode {Self::AUTH_TEST_ENDPOINT} else {Self::AUTH_LIVE_ENDPOINT}
    }

    fn get_basic_authorization(&self) -> String {
        let encoded = base64_encode(format!("{}:{}", self.client_id, self.client_secret));
        format!("Basic {}", encoded)
    }

    async fn get_bearer_authorization(&self) -> String {
        lazy_static! {
            static ref CACHE: Mutex<TtlCache<String, String>> = Mutex::new(TtlCache::new(2));
        }

        let basic_authorization = self.get_basic_authorization();

        let mut cache_guard = CACHE.lock().unwrap();

        let token = match cache_guard.get(&basic_authorization) {
            Some(value) => value.clone(),
            None => {
                let mut headers = reqwest::header::HeaderMap::new();

                headers.append("Authorization", basic_authorization.parse().unwrap());
                headers.append("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

                let request = reqwest::Client::new()
                    .request(
                        reqwest::Method::POST,
                        self.auth_url(),
                    )
                    .headers(headers)
                    .form(&[("grant_type", "client_credentials")]);

                let response = request.send().await.unwrap();

                let bytes = response.bytes().await.unwrap().to_vec();

                let data: serde_json::Value = serde_json::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();

                let access_token = data["access_token"].as_str().unwrap();
                let expires_in = data["expires_in"].as_u64().unwrap();

                cache_guard.insert(basic_authorization, access_token.to_string(), Duration::new(expires_in, 0));

                access_token.to_string()
            }
        };

        format!("Bearer {}", token)
    }
}