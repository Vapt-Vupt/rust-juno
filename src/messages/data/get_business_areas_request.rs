use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
// use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/getBusinessAreas

pub struct GetBusinessAreasRequest;

impl AbstractRequest for GetBusinessAreasRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("data/business-areas")
    }

    fn data(&self) -> Value {
        json!({})
    }
}

