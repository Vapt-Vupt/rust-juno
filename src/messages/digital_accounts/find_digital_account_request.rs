use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/findDigitalAccount

pub struct FindDigitalAccountRequest {}

impl AbstractRequest for FindDigitalAccountRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("digital-accounts")
    }

    fn data(&self) -> Value {
        json!({})
    }
}

