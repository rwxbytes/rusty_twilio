//! Applications endpoints
//! See [Applications reference](https://www.twilio.com/docs/usage/api/applications)

use super::*;
use std::string::ToString;
use strum::Display;
use crate::TwilioQuery;
use crate::url::query::ByFriendlyName;

#[derive(Clone, Debug, Deserialize)]
/// See [Application Properties](https://www.twilio.com/docs/usage/api/applications#application-properties)
pub struct ApplicationResponse {
    /// The SID of the Account that created the Application resource.
    pub account_sid: String,
    /// The API version used to start a new TwiML session.
    pub api_version: ApiVersion,
    /// The date that this account was created, in GMT in RFC 2822 format
    pub date_created: String,
    /// The date that this account was last updated, in GMT in RFC 2822 format.
    pub date_updated: String,
    /// A human-readable description of this account, up to 64 characters long. By default, the FriendlyName is your email address.
    pub friendly_name: Option<String>,
    /// The URL we call using a POST method to send message status information to your application.
    pub message_status_callback: Option<String>,
    /// The unique string that we created to identify the Application resource.
    pub sid: String,
    /// The HTTP method we use to call sms_fallback_url. Can be: GET or POST.
    pub sms_fallback_method: Option<String>,
    /// The URL that we call when an error occurs while retrieving or executing the TwiML from sms_url.
    pub sms_fallback_url: Option<String>,
    /// The HTTP method we use to call sms_url. Can be: GET or POST.
    pub sms_method: Option<String>,
    /// The URL we call using a POST method to send status information to your application about SMS messages that refer to the application.
    pub sms_status_callback: Option<String>,
    /// The URL we call when the phone number receives an incoming SMS message.
    pub sms_url: Option<String>,
    /// The URL we call using the status_callback_method to send status information to your application.
    pub status_callback: Option<String>,
    /// The HTTP method we use to call status_callback. Can be: GET or POST.
    pub status_callback_method: Option<String>,
    /// The URI for this resource, relative to https://api.twilio.com
    pub uri: String,
    /// Whether we look up the caller's caller-ID name from the CNAME database (additional charges apply). Can be: true or false.
    pub voice_caller_id_lookup: Option<bool>,
    /// The HTTP method we use to call voice_fallback_url. Can be: GET or POST.
    pub voice_fallback_method: Option<String>,
    /// The URL that we call when an error occurs retrieving or executing the TwiML requested by url.
    pub voice_fallback_url: Option<String>,
    /// The HTTP method we use to call voice_url. Can be: GET or POST.
    pub voice_method: Option<String>,
    /// The URL we call when the phone number assigned to this application receives a call.
    pub voice_url: Option<String>,
    /// Whether to allow other Twilio accounts to dial this application using Dial verb. Can be: true or false.
    pub public_application_connect_enabled: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Display)]
pub enum ApiVersion {
    #[serde(rename = "2010-04-01")]
    #[strum(to_string = "2010-04-01")]
    V20100401,
    #[serde(rename = "2008-08-01")]
    #[strum(to_string = "2008-08-01")]
    V20080801,
}

#[derive(Clone, Debug)]
pub struct CreateApplication {
    pub account_sid: String,
    pub body: CreateApplicationBody,
}

impl CreateApplication {
    pub fn new(account_sid: impl Into<String>, body: CreateApplicationBody) -> Self {
        Self {
            account_sid: account_sid.into(),
            body,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CreateApplicationBody {
    pub params: Vec<(&'static str, String)>,
}

impl CreateApplicationBody {

    pub fn with_friendly_name(mut self, friendly_name: impl Into<String>) -> Self {
        self.params
            .push((FRIENDLY_NAME, friendly_name.into()));
        self
    }

    pub fn with_api_version(mut self, api_version: ApiVersion) -> Self {
        self.params
            .push((API_VERSION, api_version.to_string()));
        self
    }

    pub fn with_voice_url(mut self, voice_url: impl Into<String>) -> Self {
        self.params.push((VOICE_URL, voice_url.into()));
        self
    }

    pub fn with_voice_method(mut self, voice_method: impl Into<String>) -> Self {
        self.params.push((VOICE_METHOD, voice_method.into()));
        self
    }

    pub fn with_voice_fallback_url(mut self, voice_fallback_url: impl Into<String>) -> Self {
        self.params.push((VOICE_FALLBACK_URL, voice_fallback_url.into()));
        self
    }

    pub fn with_voice_fallback_method(mut self, voice_fallback_method: impl Into<String>) -> Self {
        self.params.push((VOICE_FALLBACK_METHOD, voice_fallback_method.into()));
        self
    }

    pub fn with_status_callback(mut self, status_callback: impl Into<String>) -> Self {
        self.params.push((STATUS_CALLBACK, status_callback.into()));
        self
    }

    pub fn with_status_callback_method(mut self, status_callback_method: impl Into<String>) -> Self {
        self.params.push((STATUS_CALLBACK_METHOD, status_callback_method.into()));
        self
    }

    pub fn with_sms_url(mut self, sms_url: impl Into<String>) -> Self {
        self.params.push((SMS_URL, sms_url.into()));
        self
    }

    pub fn with_sms_method(mut self, sms_method: impl Into<String>) -> Self {
        self.params.push((SMS_METHOD, sms_method.into()));
        self
    }

    pub fn with_sms_fallback_url(mut self, sms_fallback_url: impl Into<String>) -> Self {
        self.params.push((SMS_FALLBACK_URL, sms_fallback_url.into()));
        self
    }

    pub fn with_sms_fallback_method(mut self, sms_fallback_method: impl Into<String>) -> Self {
        self.params.push((SMS_FALLBACK_METHOD, sms_fallback_method.into()));
        self
    }

    pub fn with_message_status_callback(mut self, message_status_callback: impl Into<String>) -> Self {
        self.params.push((MESSAGE_STATUS_CALLBACK, message_status_callback.into()));
        self
    }

    pub fn with_public_application_connect_enabled(mut self, public_application_connect_enabled: bool) -> Self {
        self.params.push((PUBLIC_APPLICATION_CONNECT_ENABLED, public_application_connect_enabled.to_string()));
        self
    }
}



impl TwilioEndpoint for CreateApplication {
    const PATH: &'static str = "2010-04-01/Accounts/{AccountSid}/Applications.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = ApplicationResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{AccountSid}", &self.account_sid)]
    }

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Form(self.body.params.clone()))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug)]
pub struct FetchApplication {
    pub account_sid: String,
    pub application_sid: String,
}

impl FetchApplication {
    pub fn new(account_sid: impl Into<String>, application_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
            application_sid: application_sid.into(),
        }
    }
}

impl TwilioEndpoint for FetchApplication {
    const PATH: &'static str = "2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ApplicationResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.application_sid),
        ]
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

impl ByFriendlyName for ListApplications {}

#[derive(Clone, Debug)]
pub struct ListApplications {
    pub account_sid: String,
    pub query: TwilioQuery<Self>,
}

impl ListApplications {
    pub fn new(account_sid: impl Into<String>, query: TwilioQuery<Self>) -> Self {
        Self {
            account_sid: account_sid.into(),
            query,
        }
    }
}

impl TwilioEndpoint for ListApplications {
    const PATH: &'static str = "2010-04-01/Accounts/{AccountSid}/Applications.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ListApplicationsResponse;

    fn query_params(&self) -> Option<QueryValues> {
        Some(self.query.params.clone())
    }

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{AccountSid}", &self.account_sid)]
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListApplicationsResponse {
    pub applications: Vec<ApplicationResponse>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Clone, Debug)]
pub struct UpdateApplication {
    pub account_sid: String,
    pub application_sid: String,
    pub body: UpdateApplicationBody,
}
pub type UpdateApplicationBody = CreateApplicationBody;

impl UpdateApplication {
    pub fn new(
        account_sid: impl Into<String>,
        application_sid: impl Into<String>,
        body: UpdateApplicationBody,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            application_sid: application_sid.into(),
            body,
        }
    }
}

impl TwilioEndpoint for UpdateApplication {
    const PATH: &'static str = "2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = ApplicationResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.application_sid),
        ]
    }

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Form(self.body.params.clone()))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug)]
pub struct DeleteApplication {
    pub account_sid: String,
    pub application_sid: String,
}

impl DeleteApplication {
    pub fn new(account_sid: impl Into<String>, application_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
            application_sid: application_sid.into(),
        }
    }
}

impl TwilioEndpoint for DeleteApplication {
    const PATH: &'static str = "2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json";

    const METHOD: Method = Method::DELETE;

    type ResponseBody = ();

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.application_sid),
        ]
    }

    async fn response_body(self, _resp: Response) -> Result<Self::ResponseBody> {
        Ok(())
    }
}
