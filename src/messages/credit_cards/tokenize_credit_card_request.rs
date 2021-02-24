use crate::errors::Error;
use crate::messages::AbstractRequest;
use crate::utils::*;
use reqwest::Method;
use serde_json::Value;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/tokenizeCreditCard](https://dev.juno.com.br/api/v2#operation/tokenizeCreditCard).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::credit_cards::TokenizeCreditCardRequest {
///     resource_token: "{resourceToken}",
///     parameters: serde_json::json!({
///       "creditCardHash": "string"
///     }),
/// };
/// let response = juno_api::request(req).await;
/// ```
pub struct TokenizeCreditCardRequest {
    pub resource_token: String,
    pub parameters: Value,
}

impl AbstractRequest for TokenizeCreditCardRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }

    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("credit-cards/tokenization")
    }

    fn data(&self) -> Result<Value, Error> {
        let params = self.parameters.clone();

        require!(params, vec!["creditCardHash",]);

        let data = params.only(&["creditCardHash"]);

        Ok(data)
    }
}
