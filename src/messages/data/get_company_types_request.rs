use crate::messages::AbstractRequest;
use reqwest::Method;
// use serde_json::*;
// use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/getCompanyTypes](https://dev.juno.com.br/api/v2#operation/getCompanyTypes).
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
pub struct GetCompanyTypesRequest;

impl AbstractRequest for GetCompanyTypesRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("data/company-types")
    }
}

