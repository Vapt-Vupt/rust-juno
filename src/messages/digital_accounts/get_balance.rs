use crate::messages::AbstractRequest;
use reqwest::Method;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/getBalance](https://dev.juno.com.br/api/v2#operation/getBalance).
//
/// # Usage example
/// ```
/// let req = juno_api::messages::digital_accounts::GetBalanceRequest {
///     resource_token: "{resourceToken}",
/// };
/// let response = juno_api::request(req).await;
/// ```
pub struct GetBalanceRequest {
    pub resource_token: String,
}

impl AbstractRequest for GetBalanceRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }

    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("balance")
    }
}
