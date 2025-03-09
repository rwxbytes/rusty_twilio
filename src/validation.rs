use base64::Engine;
use hmac::{Hmac, Mac};
use http::{HeaderMap, Method, Uri};
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
        uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("")
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

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::HeaderMap;
    use http::Method;
    use std::collections::BTreeMap;

    type HmacSha1 = Hmac<Sha1>;

    // Test utility to generate a valid Twilio signature for test cases
    fn generate_valid_signature(
        auth_token: &str,
        url: &str,
        params: Option<&BTreeMap<String, String>>,
    ) -> String {
        let mut data = url.to_string();

        // Add sorted params to the data string if they exist
        if let Some(params) = params {
            for (key, value) in params {
                data.push_str(key);
                data.push_str(value);
            }
        }

        // Compute HMAC-SHA1 signature
        let mut mac =
            HmacSha1::new_from_slice(auth_token.as_bytes()).expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes())
    }

    #[test]
    fn validate_twilio_signature_is_returning_ok_when_signature_is_valid() {
        let auth_token = "test_auth_token";
        let method = Method::POST;
        let uri = Uri::from_static("https://example.com/webhook");
        let mut headers = HeaderMap::new();
        headers.insert("Host", "example.com".parse().unwrap());

        let mut params = BTreeMap::new();
        params.insert("CallSid".to_string(), "CA123456789".to_string());
        params.insert("From".to_string(), "+12345678901".to_string());

        let signature =
            generate_valid_signature(auth_token, "https://example.com/webhook", Some(&params));
        headers.insert("X-Twilio-Signature", signature.parse().unwrap());
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8"
                .parse()
                .unwrap(),
        );

        let result = validate_twilio_signature(auth_token, &method, &uri, &headers, Some(&params));

        assert!(result.is_ok(), "Valid signature should pass validation");
    }

    #[test]
    fn validate_twilio_signature_is_returning_invalid_signature_when_signature_is_invalid() {
        let auth_token = "test_auth_token";
        let method = Method::POST;
        let uri = Uri::from_static("https://example.com/webhook");
        let mut headers = HeaderMap::new();
        headers.insert("Host", "example.com".parse().unwrap());

        headers.insert("X-Twilio-Signature", "invalid_signature".parse().unwrap());
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8"
                .parse()
                .unwrap(),
        );

        let params = BTreeMap::new();

        let result = validate_twilio_signature(auth_token, &method, &uri, &headers, Some(&params));

        assert!(result.is_err(), "Invalid signature should fail validation");
        if let Err(e) = result {
            assert!(
                matches!(e, SignatureValidationError::InvalidSignature),
                "Error should be InvalidSignature"
            );
        }
    }

    #[test]
    fn validate_twilio_signature_is_returning_missing_signature_when_signature_header_is_missing() {
        let auth_token = "test_auth_token";
        let method = Method::POST;
        let uri = Uri::from_static("https://example.com/webhook");
        let mut headers = HeaderMap::new();
        headers.insert("Host", "example.com".parse().unwrap());
        let params = BTreeMap::new();

        let result = validate_twilio_signature(auth_token, &method, &uri, &headers, Some(&params));

        assert!(result.is_err(), "Missing signature should fail validation");
        if let Err(e) = result {
            assert!(
                matches!(e, SignatureValidationError::MissingSignature),
                "Error should be MissingSignature"
            );
        }
    }

    #[test]
    fn validate_twilio_signature_is_returning_missing_host_when_host_header_is_missing() {
        let auth_token = "test_auth_token";
        let method = Method::POST;
        let uri = Uri::from_static("https://example.com/webhook");
        let mut headers = HeaderMap::new();
        let params = BTreeMap::new();
        let result = validate_twilio_signature(auth_token, &method, &uri, &headers, Some(&params));
        assert!(result.is_err(), "Missing host should fail validation");
        if let Err(e) = result {
            assert!(
                matches!(e, SignatureValidationError::MissingHost),
                "Error should be MissingHost"
            );
        }
    }

    #[test]
    fn validate_twilio_signature_is_returning_ok_when_signature_is_valid_via_get_verb() {
        let auth_token = "test_auth_token";
        let method = Method::GET;

        let url = "https://example.com/webhook?CallSid=CA123&From=%2B1234567890";
        let uri = Uri::from_static(url);

        let signature = generate_valid_signature(auth_token, url, None);

        let mut headers = HeaderMap::new();
        headers.insert("Host", "example.com".parse().unwrap());
        headers.insert("X-Twilio-Signature", signature.parse().unwrap());
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8"
                .parse()
                .unwrap(),
        );
        let result = validate_twilio_signature(auth_token, &method, &uri, &headers, None);
        assert!(result.is_ok(), "Valid signature should pass validation");
    }

    #[test]
    fn validate_twilio_signature_is_returning_invalid_signature_when_signature_is_invalid_via_get_verb(
    ) {
        let auth = "test_auth_token";
        let method = Method::GET;
        let url = "https://example.com/webhook?CallSid=CA123&From=%2B1234567890";
        let uri = Uri::from_static(url);
        let mut headers = HeaderMap::new();
        headers.insert("Host", "example.com".parse().unwrap());
        headers.insert("X-Twilio-Signature", "invalid_signature".parse().unwrap());
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8"
                .parse()
                .unwrap(),
        );
        let result = validate_twilio_signature(auth, &method, &uri, &headers, None);
        assert!(result.is_err(), "Invalid signature should fail validation");
        if let Err(e) = result {
            assert!(
                matches!(e, SignatureValidationError::InvalidSignature),
                "Error should be InvalidSignature"
            );
        }
    }
}
