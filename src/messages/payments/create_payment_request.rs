use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/createPayment

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

