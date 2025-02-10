mod client;
mod error;
pub mod endpoints;
pub mod url;

pub use client::TwilioClient;
pub use url::query::TwilioQuery;

pub type Result<T> = std::result::Result<T, error::TwilioError>;
