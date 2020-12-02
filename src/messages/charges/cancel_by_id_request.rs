use crate::messages::AbstractRequest;
use reqwest::Method;
// use serde_json::*;
// use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/cancelById](https://dev.juno.com.br/api/v2#operation/cancelById).
///
/// # Usage example
/// ```
/// let junoApi = JunoApi::with(
///     serde_json::json!({
///         "clientId": "{clientId}",
///         "clientSecret": "{clientSecret}",
///     })
/// );
/// let req = messages::charges::CancelByIdRequest {
///     resource_token: "{resourceToken}",
///     id: "{id}",
/// };
/// let response = junoApi.request(req).await;
/// ```
pub struct CancelByIdRequest {
    pub resource_token: String,
    pub id: String,
}

impl AbstractRequest for CancelByIdRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> String {
        format!("charges/{}/cancelation", self.id)
    }
}

