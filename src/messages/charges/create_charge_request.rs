use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/createCharge

pub struct CreateChargeRequest {
    pub parameters: Value,
}

impl AbstractRequest for CreateChargeRequest {
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("charges")
    }

    fn data(&self) -> Value {
        self.parameters.clone()
    }
}

