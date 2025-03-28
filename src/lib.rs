mod client;
mod client_ext;
pub mod endpoints;
pub mod error;
pub mod request_parameters;
pub mod twiml;
pub mod url;
pub mod validation;

pub use client::TwilioClient;
pub use client_ext::TwilioClientExt;
pub use url::query::TwilioQuery;

pub type Result<T> = std::result::Result<T, error::TwilioError>;
