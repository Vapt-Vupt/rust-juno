use crate::messages::abstract_request::{AbstractRequest};
use reqwest::Method;
use serde_json::*;

pub struct GetCompanyTypesRequest(pub Value);

impl AbstractRequest for GetCompanyTypesRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> &'static str {
        "data/company-types"
    }

    fn data(&self) -> Value {
        json!({})
    }
}

