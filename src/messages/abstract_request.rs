use serde_json::*;
use reqwest::*;

pub trait AbstractRequest {
    fn resource_token(&self) -> Option<String> {
        None
    }

    fn http_method(&self) -> Method;
    fn endpoint(&self) -> String;
    fn data(&self) -> Value;

    fn headers(&self) -> header::HeaderMap {
        let mut header = header::HeaderMap::new();

        header.append("X-Api-Version", "2".parse().unwrap());
        
        match self.resource_token() {
            Some(value) => {
                header.append("X-Resource-Token", value.parse().unwrap());
            }
            None => {}
        }

        header
    }
}