use crate::errors::Error;
use reqwest::{Method, header};
use serde_json::{Value, json};

/// Every Juno route needs to implement AbstractRequest.
///
/// Required implementations [http_method, endpoint].
pub trait AbstractRequest {

    /// Almost every Juno route needs a privateKey that is the resourceToken.
    ///
    /// Default to ```None```.
    ///
    /// Implement resource_token if route needs it.
    fn resource_token(&self) -> Option<&String> {
        None
    }

    /// Return http method for this route [GET, POST, PUT, DELETE, PATCH].
    ///
    /// Required implementation!
    fn http_method(&self) -> Method;

    /// Return endpoint of route.
    ///
    /// Required implementation!
    fn endpoint(&self) -> String;

    /// Work with model data to send it with the request body.
    ///
    /// Implement it if route needs body parameters.
    fn data(&self) -> Result<Value, Error> {
        Ok(json!({}))
    }

    /// Prepare header to send it with the request.
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