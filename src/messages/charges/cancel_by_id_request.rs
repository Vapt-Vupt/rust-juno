use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/cancelById

pub struct CancelByIdRequest {
    pub id: String,
}

impl AbstractRequest for CancelByIdRequest {
    
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

