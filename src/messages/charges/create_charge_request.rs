use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
use crate::utils::*;

/// Request model for route [https://dev.juno.com.br/api/v2#operation/createCharge](https://dev.juno.com.br/api/v2#operation/createCharge).
///
/// # Usage example
/// ```
/// let junoApi = JunoApi::with(
///     serde_json::json!({
///         "clientId": "{clientId}",
///         "clientSecret": "{clientSecret}",
///     })
/// );
/// let req = messages::charges::CreateChargeRequest {
///     resource_token: "{resourceToken}",
///     parameters: serde_json::json!({
///       "charge": {
///         "pixKey": "stringstringstringstringstringstring",
///         "description": "string",
///         "references": [
///           "string"
///         ],
///         "totalAmount": 0.01,
///         "amount": 0.01,
///         "dueDate": "yyyy-MM-dd",
///         "installments": 0,
///         "maxOverdueDays": 0,
///         "fine": 0,
///         "interest": "0.00",
///         "discountAmount": "0.00",
///         "discountDays": -1,
///         "paymentTypes": [
///           "string"
///         ],
///         "paymentAdvance": true,
///         "feeSchemaToken": "string",
///         "split": [
///           {
///             "recipientToken": "string",
///             "amount": 10,
///             "percentage": 10,
///             "amountRemainder": true,
///             "chargeFee": true
///           }
///         ]
///       },
///       "billing": {
///         "name": "string",
///         "document": "string",
///         "email": "string",
///         "secondaryEmail": "string",
///         "phone": "string",
///         "birthDate": "yyyy-MM-dd",
///         "notify": false
///       }
///     }),
/// };
/// let response = junoApi.request(req).await;
/// ```
pub struct CreateChargeRequest {
    pub resource_token: String,
    pub parameters: Value,
}

impl AbstractRequest for CreateChargeRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("charges")
    }

    fn data(&self) -> Value {
        let params = self.parameters.only_or_die(&[
            "charge",
            "billing",
        ]);

        let charge = params["charge"].only(&[
            "pixKey",
            "description",
            "references",
            "totalAmount",
            "amount",
            "dueDate",
            "installments",
            "maxOverdueDays",
            "fine",
            "interest",
            "discountAmount",
            "discountDays",
            "paymentTypes",
            "paymentAdvance",
            "feeSchemaToken",
            "dueDate",
            "split",
        ]);

        let billing = params["billing"].only(&[
            "name",
            "document",
            "email",
            "secondaryEmail",
            "phone",
            "birthDate",
            "notify",
        ]);

        charge.validate_or_die(&[
            "description",
            "amount"
        ]);

        billing.validate_or_die(&[
            "name",
            "document"
        ]);

        json!({
            "charge": charge,
            "billing": billing,
        })
    }
}

