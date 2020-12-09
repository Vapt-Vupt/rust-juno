
#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::JunoApi;
    use crate::messages;
    #[tokio::test]
    async fn test_connection() {
        
        let juno = JunoApi::with(
            serde_json::json!({
                "clientId": "{clientId}",
                "clientSecret": "{clientSecret}",
            })
        )
        .test_mode(true);

        let req = messages::data::GetCompanyTypesRequest;

        let response: reqwest::Response = juno.request(req).await.unwrap();

        if response.status() == 200 {
            let json: serde_json::Value = response.json().await.unwrap();
            println!("{:?}", json);
        } else {
            println!("{:?}", response.bytes().await);
        }
    }

    #[tokio::test]
    async fn test_tokenize_credit_card() {
        
        let juno = JunoApi::with(
            serde_json::json!({
                "clientId": "{}",
                "clientSecret": "{}",
            })
        )
        .test_mode(true);

        let req = messages::credit_cards::TokenizeCreditCardRequest {
            resource_token: "{}".to_string(),
            parameters: json!({
                "creditCardHash": "",
            }),
        };

        let response: reqwest::Response = juno.request(req).await.unwrap();

        if response.status() == 200 {
            let json: serde_json::Value = response.json().await.unwrap();
            println!("{:?}", json);
        } else {
            println!("{:?}", response.bytes().await);
        }
    }
}