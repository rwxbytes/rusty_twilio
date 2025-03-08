use http::{HeaderMap, Method, Uri};
use base64::Engine;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::collections::BTreeMap;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug)]
pub enum SignatureValidationError {
    MissingHost,
    MissingSignature,
    InvalidSignature,
    HmacError,
}

impl std::fmt::Display for SignatureValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHost => write!(f, "Missing Host header"),
            Self::MissingSignature => write!(f, "Missing X-Twilio-Signature header"),
            Self::InvalidSignature => write!(f, "Invalid Twilio signature"),
            Self::HmacError => write!(f, "Error computing HMAC"),
        }
    }
}

impl std::error::Error for SignatureValidationError {}

pub fn validate_twilio_signature(
    auth_token: &str,
    method: &Method,
    uri: &Uri,
    headers: &HeaderMap,
    post_params: Option<&BTreeMap<String, String>>,
) -> Result<(), SignatureValidationError> {
    // Get host from headers
    let host = headers
        .get("Host")
        .ok_or(SignatureValidationError::MissingHost)?
        .to_str()
        .map_err(|_| SignatureValidationError::InvalidSignature)?;

    // Get Twilio signature from headers
    let signature = headers
        .get("X-Twilio-Signature")
        .ok_or(SignatureValidationError::MissingSignature)?
        .to_str()
        .map_err(|_| SignatureValidationError::InvalidSignature)?;

    // Construct the base URL
    let url = format!(
        "https://{host}{}",
        uri.path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("")
    );
    let mut data = url;

    // For POST requests, add sorted parameters to the validation string
    if method == Method::POST {
        if let Some(content_type) = headers.get("Content-Type") {
            let content_type = content_type.to_str().unwrap_or("");
            if content_type.starts_with("application/x-www-form-urlencoded") {
                if let Some(params) = post_params {
                    for (key, value) in params {
                        data.push_str(key);
                        data.push_str(value);
                    }
                }
            }
        }
    }

    // Compute the HMAC-SHA1 signature
    let mut mac = HmacSha1::new_from_slice(auth_token.as_bytes())
        .map_err(|_| SignatureValidationError::HmacError)?;
    mac.update(data.as_bytes());
    let computed_signature =
        base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes());

    // Compare signatures
    if signature != computed_signature {
        return Err(SignatureValidationError::InvalidSignature);
    }

    Ok(())
}