use crate::messages::abstract_request::{AbstractRequest};
use reqwest::Method;
use serde_json::*;

pub struct FindDigitalAccountRequest(pub Value);

impl AbstractRequest for FindDigitalAccountRequest {
    
    fn http_method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> &'static str {
        "digital-accounts"
    }

    fn data(&self) -> Value {
        json!({})
    }
}

