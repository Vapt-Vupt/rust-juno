use crate::errors::Error;
use crate::messages::AbstractRequest;
use crate::utils::*;
use reqwest::Method;
use serde_json::{json, Value};
/// Request model for route [https://dev.juno.com.br/api/v2#operation/findDigitalAccount](https://dev.juno.com.br/api/v2#operation/findDigitalAccount).
///
/// # Usage example
/// ```
/// let req = juno_api::messages::digital_accounts::RequestTransferRequest {
///     resource_token: "{resourceToken}",
/// };
/// let response = juno_api::request(req).await;
/// ```
pub struct RequestTransferRequest {
    pub resource_token: String,
    pub parameters: Value,
}

impl AbstractRequest for RequestTransferRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }

    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("transfers")
    }

    fn data(&self) -> Result<Value, Error> {
        let params = self.parameters.clone();

        require!(params, vec!["type"]);
        let mut data = params.only(&["type"]);

        if let Some(type_) = params.get("type") {
            if *type_ == json!("P2P") {
                require!(
                    params,
                    vec!["type", "name", "document", "amount", "bankAccount"]
                );
                data = params.only(&["type", "name", "document", "amount", "bankAccount"]);
                require!(data["bankAccount"], vec!["accountNumber"]);
            } else if *type_ == json!("DEFAULT_BANK_ACCOUNT") {
                require!(params, vec!["type", "amount", "name"]);
                data = params.only(&["type", "name", "amount"]);
            } else if *type_ == json!("BANK_ACCOUNT") {
                require!(params, vec!["name", "document", "amount", "bankAccount"]);
                data = params.only(&["name", "document", "amount", "bankAccount"]);
                require!(
                    data["bankAccount"],
                    vec![
                        "bankNumber",
                        "agencyNumber",
                        "accountNumber",
                        "accountType",
                        "accountHolder"
                    ]
                );
                require!(
                    data["bankAccount"]["accountHolder"],
                    vec!["name", "document"]
                );
            }
        }

        Ok(data)
    }
}
