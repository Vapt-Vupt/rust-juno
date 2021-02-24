use crate::errors::Error;
use crate::messages::AbstractRequest;
use crate::utils::*;
use reqwest::Method;
use serde_json::Value;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/capturePayment](https://dev.juno.com.br/api/v2#operation/capturePayment).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::payments::CapturePaymentRequest {
///     resource_token: "{resourceToken}",
///     id: "{id}",
///     parameters: serde_json::json!({
///       "chargeId": "string",
///       "amount": 0
///     }),
/// };
/// let response = juno_api::request(req).await;
/// ```
pub struct CapturePaymentRequest {
    pub resource_token: String,
    pub id: String,
    pub parameters: Value,
}

impl AbstractRequest for CapturePaymentRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }

    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("payments/{}/capture", self.id)
    }

    fn data(&self) -> Result<Value, Error> {
        let params = self.parameters.clone();

        require!(params, vec!["chargeId", "amount",]);

        let data = params.only(&["chargeId", "amount"]);

        Ok(data)
    }
}
