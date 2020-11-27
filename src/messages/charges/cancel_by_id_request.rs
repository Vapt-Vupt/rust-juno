use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
// use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/cancelById

pub struct CancelByIdRequest {
    pub resource_token: String,
    pub id: String,
}

impl AbstractRequest for CancelByIdRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> String {
        format!("charges/{}/cancelation", self.id)
    }

    fn data(&self) -> Value {
        json!({})
    }
}

