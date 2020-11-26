use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/updateById

pub struct UpdateByIdRequest {
    pub id: String,
    pub parameters: Value,
}

impl AbstractRequest for UpdateByIdRequest {
    
    fn http_method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> String {
        format!("charges/{}/split", self.id)
    }

    fn data(&self) -> Value {
        self.parameters.clone()
    }
}

