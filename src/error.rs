use thiserror::Error;

#[derive(Error, Debug)]
pub enum TwilioError {
    #[error("request must have a body")]
    EmptyRequestBody,
    #[error("TWILIO_ACCOUNT_SID not set")]
    MissingAccountSidEnvVar,
    #[error("TWILIO_AUTH_TOKEN not set")]
    MissingAuthTokenEnvVar,
    #[error("http error")]
    Request(#[from] reqwest::Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
    #[error("http error {0}")]
    Http(serde_json::Value),
}