
#[cfg(test)]
mod tests {
    use serde_json::json;
    use dotenv::dotenv;
    use crate::{messages, request};
    #[tokio::test]
    async fn test_connection() {
        dotenv().ok();

        let req = messages::data::GetCompanyTypesRequest;

        let response: reqwest::Response = request(req).await.unwrap();

        if response.status() == 200 {
            let json: serde_json::Value = response.json().await.unwrap();
            println!("{:?}", json);
        } else {
            println!("{:?}", response.bytes().await);
        }
    }

    #[tokio::test]
    async fn test_tokenize_credit_card() {
        dotenv().ok();

        let req = messages::credit_cards::TokenizeCreditCardRequest {
            resource_token: "{}".to_string(),
            parameters: json!({
                "creditCardHash": "",
            }),
        };

        let response: reqwest::Response = request(req).await.unwrap();

        if response.status() == 200 {
            let json: serde_json::Value = response.json().await.unwrap();
            println!("{:?}", json);
        } else {
            println!("{:?}", response.bytes().await);
        }
    }
}