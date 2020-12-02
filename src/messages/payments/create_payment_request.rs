use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/createPayment](https://dev.juno.com.br/api/v2#operation/createPayment).
///
/// # Usage example
/// ```
/// let junoApi = JunoApi::with(
///     serde_json::json!({
///         "clientId": "{clientId}",
///         "clientSecret": "{clientSecret}",
///     })
/// );
/// let req = messages::payments::CreatePaymentRequest {
///     resource_token: "{resourceToken}",
///     parameters: serde_json::json!({
///       "chargeId": "string",
///       "billing": {
///         "email": "string",
///         "address": {
///           "street": "string",
///           "number": "string",
///           "complement": "string",
///           "neighborhood": "string",
///           "city": "string",
///           "state": "string",
///           "postCode": "string"
///         },
///         "delayed": true
///       },
///       "creditCardDetails": {
///         "creditCardId": "string",
///         "creditCardHash": "string"
///       }
///     }),
/// }
/// let response = junoApi.request(req).await;
/// ```
pub struct CreatePaymentRequest {
    pub resource_token: String,
    pub parameters: Value,
}

impl AbstractRequest for CreatePaymentRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("payments")
    }

    fn data(&self) -> Value {
        let mut params = self.parameters.only_or_die(&[
            "chargeId",
            "billing",
            "creditCardDetails",
        ]);

        params["billing"].validate_or_die(&[
            "email",
            "address",
            "delayed",
        ]);

        params["billing"]["address"].validate_or_die(&[
            "street",
            "city",
            "state",
            "postCode",
        ]);

        params["billing"]["address"]["number"] = params["billing"]["address"].get("number").unwrap_or(&json!("N/A")).clone();

        params["creditCardDetails"].validate_or_die(&[
            "creditCardId",
            "creditCardHash",
        ]);

        params
    }
}

