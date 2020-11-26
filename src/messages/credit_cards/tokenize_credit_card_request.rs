use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;

// https://dev.juno.com.br/api/v2#operation/tokenizeCreditCard

pub struct TokenizeCreditCardRequest {
    pub parameters: Value,
}

impl AbstractRequest for TokenizeCreditCardRequest {
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("credit-cards/tokenization")
    }

    fn data(&self) -> Value {
        self.parameters.clone()
    }
}

