use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/createPayment

pub struct CreatePaymentRequest {
    pub parameters: Value,
}

impl AbstractRequest for CreatePaymentRequest {
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("payments")
    }

    fn data(&self) -> Value {
        self.parameters.clone()
    }
}

