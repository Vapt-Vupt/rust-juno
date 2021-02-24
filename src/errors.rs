use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum Error {
    JunoInternalServerError(ReqwestError),
    InvalidAuthorization(ReqwestError),
    MissingRequiredFields(Vec<&'static str>),
    JsonDeserialization,
    Response(ReqwestError),
}
