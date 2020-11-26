use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/capturePayment

pub struct CapturePaymentRequest {
    pub id: String,
    pub parameters: Value,
}

impl AbstractRequest for CapturePaymentRequest {
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("payments/{}/capture", self.id)
    }

    fn data(&self) -> Value {
        self.parameters.clone()
    }
}

