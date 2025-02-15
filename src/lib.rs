mod client;
pub mod error;
pub mod endpoints;
pub mod url;
pub mod twiml;

pub use client::TwilioClient;
pub use url::query::TwilioQuery;

pub type Result<T> = std::result::Result<T, error::TwilioError>;
