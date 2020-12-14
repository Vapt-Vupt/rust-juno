use crate::errors::Error;
use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::Value;
use serde_json::json;
use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/findDigitalAccount](https://dev.juno.com.br/api/v2#operation/findDigitalAccount).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::digital_accounts::FindDigitalAccountRequest {
///     resource_token: "{resourceToken}",
/// };
/// let response = juno_api::request(req).await;
/// ```
pub struct FindDigitalAccountRequest {
    pub resource_token: String,
}

impl AbstractRequest for FindDigitalAccountRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("digital-accounts")
    }
}

