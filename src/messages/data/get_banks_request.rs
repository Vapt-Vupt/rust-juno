use crate::errors::Error;
use crate::messages::AbstractRequest;
use crate::utils::*;
use reqwest::Method;
use serde_json::json;
use serde_json::Value;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/getBanks](https://dev.juno.com.br/api/v2#operation/getBanks).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::data::GetBanksRequest;
/// let response = juno_api::request(req).await;
/// ```
pub struct GetBanksRequest;

impl AbstractRequest for GetBanksRequest {
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("data/banks")
    }
}
