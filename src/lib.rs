
pub mod messages;
pub mod juno_api;


#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_connection() {
        use crate::juno_api;
        use crate::messages;

        let juno = juno_api::JunoApi::new("client_id", "client_secret")
            .test_mode(true);

        let req = messages::GetCompanyTypesRequest (
            serde_json::json!({}),
        );

        let response = juno.request(req).await.unwrap();

        println!("{:?}", response.bytes().await);
    }
}
