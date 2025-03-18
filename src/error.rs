use serde::Deserialize;
use crate::validation::SignatureValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TwilioError {
    #[error("API error ({status}): {error:?}")]
    Api {
        status: reqwest::StatusCode,
        error: TwilioApiError,
    },
    #[error("request must have a body")]
    EmptyRequestBody,
    #[error("TWILIO_ACCOUNT_SID not set")]
    MissingAccountSidEnvVar,
    #[error("TWILIO_AUTH_TOKEN not set")]
    MissingAuthTokenEnvVar,
    #[error("TWILIO_PHONE_NUMBER not set")]
    MissingPhoneNumberEnvVar,
    #[error("http error")]
    Request(#[from] reqwest::Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
    #[error("invalid websocket url: {0}")]
    InvalidWebSocketUrl(String),
    #[error("invalid callback url: {0}")]
    InvalidCallbackUrl(String),
    #[error("XML writing error: {0}")]
    Xml(#[from] xml::writer::Error),
    #[error("UTF-8 encoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("signature validation error: {0}")]
    Validation(#[from] SignatureValidationError),
    #[error("unsupported noun")]
    UnsupportedNoun,
}


#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TwilioApiError {
    code: i32,
    message: String,
    more_info: String,
    status: i32,
}
