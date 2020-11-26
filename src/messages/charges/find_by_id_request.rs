use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/findById

pub struct FindByIdRequest {
    pub id: String,
}

impl AbstractRequest for FindByIdRequest {
    
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

