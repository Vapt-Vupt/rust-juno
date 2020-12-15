#[macro_use]
mod utils;
pub mod errors;
pub mod messages;
mod tests;

use base64::{encode as base64_encode};
use std::{time::Duration};
use ttl_cache::TtlCache;
use lazy_static::*;
use mut_static::MutStatic;
use crate::messages::AbstractRequest;
use crate::utils::*;
use crate::errors::*;

/// Juno api requires 2 fields: clientId and clientSecret to operate.
///
/// Every request implements AbstractRequest.
/// To use this api with custom routes you need to implement AbstractRequest.
///
/// # Usage example
/// ```
/// let req = juno_api::messages::data::GetBanksRequest;
///
/// // The pub function request needs you to set env vars [JUNO_TEST_MODE, JUNO_CLIENT_ID, JUNO_CLIENT_SECRET].
/// // [JUNO_TEST_CLIENT_ID, JUNO_TEST_CLIENT_SECRET] are for test mode.
/// let response: reqwest::Response = juno_api::request(req).await.unwrap();
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
    ) -> Result<reqwest::Response, Error>  {
        let mut headers = reqwest::header::HeaderMap::new();

        let bearer_token = self.get_bearer_authorization().await?;

        headers.append("Authorization", bearer_token.parse().unwrap());
        headers.append("Content-Type", "application/json".parse().unwrap());
        headers.extend(req.headers());

        let url = format!("{}/{}", self.base_url(), req.endpoint());

        let data = req.data()?;

        let request = reqwest::Client::new()
            .request(
                req.http_method(),
                reqwest::Url::parse(url.as_str()).unwrap(),
            )
            .headers(headers)
            .body(serde_json::to_string(&data).unwrap());

        let response = request.send().await;

        if response.is_err() {
            let error = response.err().unwrap();
            let status = error.status().unwrap();

            return Err(
                match status {
                    reqwest::StatusCode::UNAUTHORIZED => Error::InvalidAuthorization(error),
                    reqwest::StatusCode::INTERNAL_SERVER_ERROR => Error::JunoInternalServerError(error),
                    _ => Error::Response(error),
                }
            );
        }

        Ok(response.unwrap())
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

    async fn get_bearer_authorization(&self) -> Result<String, Error> {
        lazy_static! {
            static ref CACHE: MutStatic<TtlCache<String, String>> = {
                let cache: MutStatic<TtlCache<String, String>> = MutStatic::new();
                cache.set(TtlCache::new(2)).unwrap();
                cache
            };
        }

        let basic_authorization = self.get_basic_authorization();

        let maybe_token: Option<String>;
        {
            let cache_read = CACHE.read().unwrap();
            let token = cache_read.get(&basic_authorization);
            
            maybe_token = match token {
                Some(v) => Some(v.clone()),
                None => None,
            };
        }

        let token = match maybe_token {
            Some(value) => Ok(value.clone()),
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

                let response = request.send().await;

                if response.is_err() {
                    let error = response.err().unwrap();
                    let status = error.status().unwrap();

                    return Err(
                        match status {
                            reqwest::StatusCode::UNAUTHORIZED => Error::InvalidAuthorization(error),
                            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Error::JunoInternalServerError(error),
                            _ => Error::Response(error),
                        }
                    );
                }

                let serialization = response.unwrap().json().await;

                if serialization.is_err() {
                    return Err(Error::JsonDeserialization);
                }

                let data: serde_json::Value = serialization.unwrap();

                let fields = vec!["access_token", "expires_in"];

                require!(data, fields);

                let access_token = data["access_token"].as_str().unwrap();
                let expires_in = data["expires_in"].as_u64().unwrap();

                {
                    let mut cache_write = CACHE.write().unwrap();
                    cache_write.insert(basic_authorization, access_token.to_string(), Duration::new(expires_in, 0));
                }

                Ok(access_token.to_string())
            }
        }?;

        Ok(format!("Bearer {}", token))
    }
}

pub async fn request(
    req: impl AbstractRequest,
) -> Result<reqwest::Response, Error> {
    use std::env::var;

    let test_mode: bool = var("JUNO_TEST_MODE").unwrap_or("false".to_string()).parse().unwrap();
    
    let (client_id, client_secret) = 
        if test_mode {
            (
                var("JUNO_TEST_CLIENT_ID").expect("Missing JUNO_TEST_CLIENT_ID from env"),
                var("JUNO_TEST_CLIENT_SECRET").expect("Missing JUNO_TEST_CLIENT_SECRET from env"),
            )
        } else {
            (
                var("JUNO_CLIENT_ID").expect("Missing JUNO_CLIENT_ID from env"),
                var("JUNO_CLIENT_SECRET").expect("Missing JUNO_CLIENT_SECRET from env"),
            )
        };

    let juno = JunoApi {
        test_mode,
        client_id,
        client_secret,
    };

    juno.request(req).await
}