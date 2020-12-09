use crate::errors::Error;
use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::Value;
use serde_json::json;
use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/getBusinessAreas](https://dev.juno.com.br/api/v2#operation/getBusinessAreas).
///
/// # Usage example
/// ```
/// let junoApi = JunoApi::with(
///     serde_json::json!({
///         "clientId": "{clientId}",
///         "clientSecret": "{clientSecret}",
///     })
/// );
/// let req = messages::data::CreateChargeRequest;
/// let response = junoApi.request(req).await;
/// ```
pub struct GetBusinessAreasRequest;

impl AbstractRequest for GetBusinessAreasRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("data/business-areas")
    }
}

