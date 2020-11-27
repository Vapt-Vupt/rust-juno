use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
// use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/findById

pub struct FindByIdRequest {
    pub resource_token: String,
    pub id: String,
}

impl AbstractRequest for FindByIdRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("charges/{}", self.id)
    }

    fn data(&self) -> Value {
        json!({})
    }
}

