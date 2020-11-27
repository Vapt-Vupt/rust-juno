use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/capturePayment

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

    fn data(&self) -> Value {
        self.parameters.only_or_die(&[
            "chargeId",
            "amount",
        ])
    }
}

