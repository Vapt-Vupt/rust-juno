use crate::errors::Error;
use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::Value;
use serde_json::json;
use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/updateById](https://dev.juno.com.br/api/v2#operation/updateById).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::charges::UpdateByIdRequest {
///     resource_token: "{resourceToken}",
///     id: "{id}",
///     parameters: serde_json::json!({
///       "split": [
///         {
///           "recipientToken": "string",
///           "amount": 10,
///           "percentage": 10,
///           "amountRemainder": true,
///           "chargeFee": true
///         }
///       ]
///     }),
/// };
/// let response = juno_api::request(req).await;
/// ```
pub struct UpdateByIdRequest {
    pub resource_token: String,
    pub id: String,
    pub parameters: Value,
}

impl AbstractRequest for UpdateByIdRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> String {
        format!("charges/{}/split", self.id)
    }

    fn data(&self) -> Result<Value, Error> {
        let params = self.parameters.clone();

        require!(params, vec![
            "split",
        ]);

        let data = params.only(&[
            "split",
        ]);

        Ok(data)
    }
}

