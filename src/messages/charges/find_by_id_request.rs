use crate::errors::Error;
use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::Value;
use serde_json::json;
use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/findById](https://dev.juno.com.br/api/v2#operation/findById).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::charges::FindByIdRequest {
///     resource_token: "{resourceToken}",
///     id: "{id}",
/// };
/// let response = juno_api::request(req).await;
/// ```
pub struct FindByIdRequest {
    pub resource_token: String,
    pub id: String,
}

impl AbstractRequest for FindByIdRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("charges/{}", self.id)
    }
}

