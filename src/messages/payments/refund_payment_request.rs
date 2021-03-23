use crate::errors::Error;
use crate::messages::AbstractRequest;
use crate::utils::*;
use reqwest::Method;
use serde_json::Value;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/createPayment](https://dev.juno.com.br/api/v2#operation/createPayment).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::payments::RefundPaymentRequest {
///    resource_token: "{resourceToken}",
///     id: "{id}",
///     parameters: serde_json::json!({
///       "amount": "string",
///       "split": [
///           {
///             "recipientToken": "string",
///             "amount": 10,
///             "percentage": 10,
///             "amountRemainder": true,
///             "chargeFee": true
///           }
///         ]
///     }),
/// }
/// let response = juno_api::request(req).await;
/// ```
pub struct RefundPaymentRequest {
    pub id: String,
    pub resource_token: String,
    pub parameters: Value,
}

impl AbstractRequest for RefundPaymentRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }

    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("payments/{}/refunds", self.id)
    }

    fn data(&self) -> Result<Value, Error> {
        let params = self.parameters.clone();
        require!(params, vec!["amount", "split",]);

        let data = params.only(&["amount", "split"]);

        Ok(data)
    }
}
