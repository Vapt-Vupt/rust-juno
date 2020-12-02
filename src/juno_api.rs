use serde_json;
use base64::{encode as base64_encode};
use std::{time::Duration};
use ttl_cache::TtlCache;
use lazy_static::*;
use std::sync::Mutex;
use crate::messages::AbstractRequest;
use crate::utils::*;

/// Juno api requires 2 fields: clientId and clientSecret to operate.
///
/// Every request implements AbstractRequest.
/// To use this api with custom routes you need to implement AbstractRequest.
///
/// # Usage example
/// ```
/// let junoApi = JunoApi::with(
///     serde_json::json!({
///         "clientId": "{clientId}",
///         "clientSecret": "{clientSecret}",
///     })
/// );
/// let req = messages::data::GetBanksRequest;
/// let response: reqwest::Response = juno.request(req).await.unwrap();
/// 
/// if response.status() == 200 {
///     let json: serde_json::Value = response.json().await.unwrap();
///     println!("{:?}", json);
/// } else {
///     println!("{:?}", response.bytes().await);
/// }
/// ```
pub struct JunoApi {
    test_mode: bool,
    client_id: String,
    client_secret: String,
}

impl JunoApi {
    const LIVE_BASE_URL: &'static str       = "https://api.juno.com.br";
    const TEST_BASE_URL: &'static str       = "https://sandbox.boletobancario.com/api-integration";
    const AUTH_LIVE_ENDPOINT: &'static str  = "https://api.juno.com.br/authorization-server/oauth/token";
    const AUTH_TEST_ENDPOINT: &'static str  = "https://sandbox.boletobancario.com/authorization-server/oauth/token";

    /// Insert json args: {clientId, clientSecret, testMode}.
    pub fn with(config: serde_json::Value) -> Self {
        config.validate_or_die(&["clientId", "clientSecret"]);
        Self {
            test_mode: config["testMode"].as_bool().unwrap_or(false),
            client_id: config["clientId"].as_str().unwrap().into(),
            client_secret: config["clientSecret"].as_str().unwrap().into(),
        }
    }

    /// Set testMode: false = production endpoint, true = sandbox endpoint.
    pub fn test_mode(mut self, value: bool) -> Self {
        self.test_mode = value;
        self
    }

    /// Returns reqwest::Reponse from ```impl AbstractRequest```
    ///
    /// # Arguments
    ///
    /// * `req` - A request instance that implements AbstractRequest trait.
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

                let response = request.send().await.expect("Bearer token get error.");

                let data: serde_json::Value = response.json().await.expect("Received body is not in json format.");

                data.validate_or_die(&["access_token", "expires_in"]);

                let access_token = data["access_token"].as_str().unwrap();
                let expires_in = data["expires_in"].as_u64().unwrap();

                cache_guard.insert(basic_authorization, access_token.to_string(), Duration::new(expires_in, 0));

                access_token.to_string()
            }
        };

        format!("Bearer {}", token)
    }
}