use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/getBanks

pub struct GetBanksRequest {}

impl AbstractRequest for GetBanksRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("data/banks")
    }

    fn data(&self) -> Value {
        json!({})
    }
}

