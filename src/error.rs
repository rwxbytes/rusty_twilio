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
    #[error("invalid websocket url: {0}")]
    InvalidWebSocketUrl(String),
    #[error("XML writing error: {0}")]
    Xml(#[from] xml::writer::Error),
    #[error("UTF-8 encoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

}