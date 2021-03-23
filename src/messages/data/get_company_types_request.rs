use crate::errors::Error;
use crate::messages::AbstractRequest;
use crate::utils::*;
use reqwest::Method;
use serde_json::json;
use serde_json::Value;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/getCompanyTypes](https://dev.juno.com.br/api/v2#operation/getCompanyTypes).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::data::CreateChargeRequest;
/// let response = juno_api::request(req).await;
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
