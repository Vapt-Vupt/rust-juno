use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
// use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/getCompanyTypes

pub struct GetCompanyTypesRequest;

impl AbstractRequest for GetCompanyTypesRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("data/company-types")
    }

    fn data(&self) -> Value {
        json!({})
    }
}

