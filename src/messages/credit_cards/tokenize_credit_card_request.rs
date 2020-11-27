use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/tokenizeCreditCard

pub struct TokenizeCreditCardRequest {
    pub resource_token: String,
    pub parameters: Value,
}

impl AbstractRequest for TokenizeCreditCardRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("credit-cards/tokenization")
    }

    fn data(&self) -> Value {
        let params = self.parameters.only_or_die(&[
            "creditCardHash",
        ]);

        params
    }
}

