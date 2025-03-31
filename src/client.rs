#![allow(dead_code)]
use crate::endpoints::TwilioEndpoint;
use crate::error::*;
use crate::validation::*;
use crate::Result;
use http::{HeaderMap, Method, Uri};
use std::collections::BTreeMap;
use url::Url;

const APPLICATION_JSON: &str = "application/json";

#[derive(Clone, Debug)]
pub struct TwilioClient {
    inner: reqwest::Client,
    account_sid: String,
    auth_token: String,
    main_api_key: Option<String>,
    main_api_key_secret: Option<String>,
    number: Option<String>,
    base_url: Url,
}

impl TwilioClient {
    pub fn account_sid(&self) -> &str {
        &self.account_sid
    }

    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self {
            inner: reqwest::Client::new(),
            account_sid: std::env::var("TWILIO_ACCOUNT_SID")
                .map_err(|_| TwilioError::MissingAccountSidEnvVar)?,
            auth_token: std::env::var("TWILIO_AUTH_TOKEN")
                .map_err(|_| TwilioError::MissingAuthTokenEnvVar)?,
            main_api_key: std::env::var("TWILIO_MAIN_API_KEY").ok(),
            main_api_key_secret: std::env::var("TWILIO_MAIN_API_KEY_SECRET").ok(),
            number: std::env::var("TWILIO_PHONE_NUMBER").ok(),
            base_url: Url::parse("https://api.twilio.com").unwrap(),
        })
    }

    pub fn new(account_sid: impl Into<String>, auth_token: impl Into<String>) -> Self {
        Self {
            inner: reqwest::Client::new(),
            account_sid: account_sid.into(),
            auth_token: auth_token.into(),
            main_api_key: None,
            main_api_key_secret: None,
            number: None,
            base_url: Url::parse("https://api.twilio.com").unwrap(),
        }
    }

    pub async fn hit<E: TwilioEndpoint>(&self, endpoint: E) -> Result<E::ResponseBody> {
        let mut builder = self
            .inner
            .request(E::METHOD, endpoint.url(&self.base_url))
            .basic_auth(&self.account_sid, Some(&self.auth_token));

        builder = endpoint.configure_request_body(builder)?;

        let resp = builder.send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let error: TwilioApiError = resp.json().await?;
            return Err(TwilioError::Api { status, error });
        }

        E::response_body(resp).await
    }

    pub fn number(&self) -> Option<&str> {
        self.number.as_deref()
    }
    pub fn with_number(mut self, number: impl Into<String>) -> Self {
        self.number = Some(number.into());
        self
    }

    pub fn with_base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn validate_request(
        &self,
        method: &Method,
        uri: &Uri,
        headers: &HeaderMap,
        post_params: Option<&BTreeMap<String, String>>,
    ) -> Result<()> {
        Ok(validate_twilio_signature(
            &self.auth_token,
            method,
            uri,
            headers,
            post_params,
        )?)
    }
}
