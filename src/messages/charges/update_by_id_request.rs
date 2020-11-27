use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/updateById

pub struct UpdateByIdRequest {
    pub resource_token: String,
    pub id: String,
    pub parameters: Value,
}

impl AbstractRequest for UpdateByIdRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> String {
        format!("charges/{}/split", self.id)
    }

    fn data(&self) -> Value {
        let params = self.parameters.only_or_die(&[
            "split",
        ]);

        params
    }
}

