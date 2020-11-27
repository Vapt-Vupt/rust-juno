use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
// use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/findDigitalAccount

pub struct FindDigitalAccountRequest {
    pub resource_token: String,
}

impl AbstractRequest for FindDigitalAccountRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
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

