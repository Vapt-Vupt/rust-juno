pub mod utils;
pub mod messages;
pub mod juno_api;

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_connection() {
        use crate::juno_api::*;
        use crate::messages;

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
}
