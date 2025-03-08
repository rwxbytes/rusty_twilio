use crate::endpoints::{Region, RequestBody, TwilioEndpoint};
use crate::error::TwilioError::*;
use crate::validation::*;
use crate::Result;
use http::{HeaderMap, Method, Uri};
use reqwest::header::CONTENT_TYPE;
use std::collections::BTreeMap;

const APPLICATION_JSON: &str = "application/json";

#[derive(Clone, Debug)]
pub struct TwilioClient {
    inner: reqwest::Client,
    account_sid: String,
    auth_token: String,
    main_api_key: Option<String>,
    main_api_key_secret: Option<String>,
    number: Option<String>,
    region: Region,
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
                .map_err(|_| MissingAccountSidEnvVar)?,
            auth_token: std::env::var("TWILIO_AUTH_TOKEN").map_err(|_| MissingAuthTokenEnvVar)?,
            main_api_key: std::env::var("TWILIO_MAIN_API_KEY").ok(),
            main_api_key_secret: std::env::var("TWILIO_MAIN_API_KEY_SECRET").ok(),
            number: std::env::var("TWILIO_PHONE_NUMBER").ok(),
            region: Region::UnitedStates,
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
            region: Region::UnitedStates,
        }
    }

    pub async fn hit<T: TwilioEndpoint>(&self, endpoint: T) -> Result<T::ResponseBody> {
        //if self.region != Default::default()

        let mut builder = self
            .inner
            .request(T::METHOD, endpoint.url())
            .basic_auth(&self.account_sid, Some(&self.auth_token));

        if matches!(T::METHOD, Method::POST | Method::PATCH) {
            let request_body = endpoint.request_body()?;
            builder = match request_body {
                RequestBody::Json(json) => {
                    builder.header(CONTENT_TYPE, APPLICATION_JSON).json(&json)
                }
                RequestBody::Multipart(form) => builder.multipart(form),
                RequestBody::Form(form) => builder.form(&form),
                RequestBody::Empty => return Err(EmptyRequestBody),
            };
        }

        let resp = builder.send().await?;

        if !resp.status().is_success() {
            return Err(Http(resp.json().await?));
        }

        endpoint.response_body(resp).await
    }

    pub fn number(&self) -> Option<&str> {
        self.number.as_deref()
    }

    pub fn set_region(&mut self, region: Region) {
        self.region = region;
    }

    pub fn validate_request(
        &self,
        method: &Method,
        uri: &Uri,
        headers: &HeaderMap,
        post_params: Option<&BTreeMap<String, String>>,
    ) -> Result<()> {
        Ok(validate_twilio_signature(&self.auth_token, method, uri, headers, post_params)?)
    }
}
