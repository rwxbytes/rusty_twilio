mod client;
mod error;
pub mod endpoints;

pub use client::TwilioClient;

pub type Result<T> = std::result::Result<T, error::TwilioError>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
