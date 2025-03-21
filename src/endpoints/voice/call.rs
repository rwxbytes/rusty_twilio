//! Call endpoints
//! See [Call Resource reference](https://www.twilio.com/docs/voice/api/call-resource)
#![allow(unused_imports)]
use super::*;
use crate::endpoints::applications::ApiVersion;
use crate::url::query::{ByToAndFrom, CallQueryMarker, TwilioQuery};
use std::collections::HashMap;
use std::string::ToString;
use strum::Display;

#[derive(Clone, Debug, Deserialize)]
/// See [Twilio's Request To Your Application](https://www.twilio.com/docs/voice/twiml#twilios-request-to-your-application)
#[serde(rename_all = "PascalCase")]
pub struct TwilioRequestParams {
    pub call_sid: String,
    pub account_sid: String,
    pub from: String,
    pub to: String,
    pub call_status: CallStatus,
    pub api_version: ApiVersion,
    pub direction: String,
    pub forwarded_from: Option<String>,
    pub caller_name: Option<String>,
    pub parent_call_sid: Option<String>,
    pub call_token: Option<String>,
    pub from_city: Option<String>,
    pub from_state: Option<String>,
    pub from_zip: Option<String>,
    pub from_country: Option<String>,
    pub to_city: Option<String>,
    pub to_state: Option<String>,
    pub to_zip: Option<String>,
    pub to_country: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
/// See [Call Properties](https://www.twilio.com/docs/voice/api/call-resource#call-properties)
pub struct CallResponse {
    /// The unique string that we created to identify this Call resource.
    pub sid: String,
    /// The date and time in UTC that this resource was created specified in RFC 2822 format.
    pub date_created: Option<String>,
    /// The date and time in UTC that this resource was last updated, specified in RFC 2822 format.
    pub date_updated: Option<String>,
    /// The SID that identifies the call that created this leg.
    pub parent_call_sid: Option<String>,
    /// The SID of the Account that created this Call resource.
    pub account_sid: String,
    /// The phone number, SIP address, Client identifier or SIM SID that received this call.
    /// Phone numbers are in E.164 format (e.g., +16175551212). SIP addresses are formatted
    /// as `name@company.com` Client identifiers are formatted `client:name`. SIM SIDs are
    /// formatted as `sim:sid`.
    pub to: String,
    /// The phone number, SIP address or Client identifier that received this call. Formatted
    /// for display. Non-North American phone numbers are in E.164 format (e.g., +442071838750).
    pub to_formatted: Option<String>,
    /// The phone number, SIP address, Client identifier or SIM SID that made this call. Phone
    /// numbers are in E.164 format (e.g., +16175551212). SIP addresses are formatted as
    /// `name@company.com`. Client identifiers are formatted `client:name`. SIM SIDs are
    /// formatted as `sim:sid`.
    pub from: String,
    /// The calling phone number, SIP address, or Client identifier formatted for display.
    /// Non-North American phone numbers are in E.164 format (e.g., +442071838750).
    pub from_formatted: Option<String>,
    /// If the call was inbound, this is the SID of the IncomingPhoneNumber resource that
    /// received the call. If the call was outbound, it is the SID of the OutgoingCallerId
    /// resource from which the call was placed.
    pub phone_number_sid: Option<String>,
    /// The status of this call. Can be: queued, ringing, in-progress, canceled, completed,
    /// failed, busy or no-answer.
    pub status: Option<CallStatus>,
    /// The start time of the call, given as UTC in RFC 2822 format. Empty if the call has not
    /// yet been dialed.
    pub start_time: Option<String>,
    /// The time the call ended, given as UTC in RFC 2822 format. Empty if the call did not
    /// complete successfully.
    pub end_time: Option<String>,
    /// The length of the call in seconds. This value is empty for busy, failed, unanswered,
    /// or ongoing calls.
    pub duration: Option<String>,
    /// The charge for this call, in the currency associated with the account. Populated after
    /// the call is completed. May not be immediately available. The price associated with a
    /// call only reflects the charge for connectivity. Charges for other call-related features
    /// such as Answering Machine Detection, Text-To-Speech, and SIP REFER are not included in
    /// this value.
    pub price: Option<String>,
    /// The currency in which Price is measured, in ISO 4127 format (e.g., USD, EUR, JPY).
    /// Always capitalized for calls.
    pub price_unit: Option<String>,
    /// A string describing the direction of the call. Can be: inbound for inbound calls,
    /// outbound-api for calls initiated via the REST API or outbound-dial for calls initiated
    /// by a <Dial> verb. Using Elastic SIP Trunking, the values can be trunking-terminating
    /// for outgoing calls from your communications infrastructure to the PSTN or
    /// trunking-originating for incoming calls to your communications infrastructure from the PSTN.
    pub direction: Option<String>,
    /// Either human or machine if this call was initiated with answering machine detection.
    /// Empty otherwise.
    pub answered_by: Option<String>,
    /// The API version used to create the call.
    pub api_version: Option<ApiVersion>,
    /// The forwarding phone number if this call was an incoming call forwarded from another
    /// number (depends on carrier supporting forwarding). Otherwise, empty.
    pub forwarded_from: Option<String>,
    /// The Group SID associated with this call. If no Group is associated with the call, the
    /// field is empty.
    pub group_sid: Option<String>,
    /// The caller's name if this call was an incoming call to a phone number with caller ID
    /// Lookup enabled. Otherwise, empty.
    pub caller_name: Option<String>,
    /// The wait time in milliseconds before the call is placed.
    pub queue_time: Option<String>,
    /// The unique identifier of the trunk resource that was used for this call. The field is
    /// empty if the call was not made using a SIP trunk or if the call is not terminated.
    pub trunk_sid: Option<String>,
    /// The URI of this resource, relative to https://api.twilio.com.
    pub uri: String,
    /// A list of subresources available to this call, identified by their URIs relative to
    /// https://api.twilio.com.
    //pub subresource_uris: Option<UriMap>,
    pub subresource_uris: Option<serde_json::Value>,
    pub annotation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Display, Serialize)]
/// See [Call Status](https://www.twilio.com/docs/voice/api/call-resource#call-status-values)
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum CallStatus {
    Queued,
    Ringing,
    InProgress,
    Canceled,
    Completed,
    Failed,
    Busy,
    NoAnswer,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnsweredBy {
    Human,
    Machine,
    MachineStart,
    Unknown,
}

#[derive(Debug)]
pub struct CreateCall<'a> {
    pub account_sid: String,
    pub body: RequestBody<CreateCallBody<'a>>,
}

impl<'a> CreateCall<'a> {
    pub fn new(account_sid: impl Into<String>, body: CreateCallBody<'a>) -> Self {
        Self {
            account_sid: account_sid.into(),
            body: RequestBody::Form(body),
        }
    }
}

impl<'a> CreateCallBody<'a> {
    pub fn new(to: &'a str, from: &'a str, url: &'a str) -> Self {
        Self {
            to,
            from,
            url: Some(url),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCallBody<'a> {
    pub to: &'a str,
    pub from: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twiml: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_sid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<&'a str>,
    #[serde(
        rename = "StatusCallbackEvent",
        serialize_with = "StatusCallbackEvent::serialize_initiated",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_event_initiated: Option<bool>,
    #[serde(
        rename = "StatusCallbackEvent",
        serialize_with = "StatusCallbackEvent::serialize_ringing",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_event_ringing: Option<bool>,
    #[serde(
        rename = "StatusCallbackEvent",
        serialize_with = "StatusCallbackEvent::serialize_answered",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_event_answered: Option<bool>,
    #[serde(
        rename = "StatusCallbackEvent",
        serialize_with = "StatusCallbackEvent::serialize_completed",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_event_completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_digits: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_channels: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_status_callback: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_auth_username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_auth_password: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_timeout: Option<u32>,
    #[serde(
        rename = "RecordingStatusCallbackEvent",
        serialize_with = "RecordingStatusCallbackEvent::serialize_in_progress",
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_status_callback_event_in_progress: Option<bool>,
    #[serde(
        rename = "RecordingStatusCallbackEvent",
        serialize_with = "RecordingStatusCallbackEvent::serialize_completed",
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_status_callback_event_completed: Option<bool>,
    #[serde(
        rename = "RecordingStatusCallbackEvent",
        serialize_with = "RecordingStatusCallbackEvent::serialize_absent",
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_status_callback_event_absent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_speech_threshold: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_speech_end_threshold: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_silence_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_amd: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_amd_status_callback: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_amd_status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoc: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_reason: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_track: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<u32>,
}

impl TwilioEndpoint for CreateCall<'_> {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Calls.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = CallResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{AccountSid}", &self.account_sid)]
    }

    fn configure_request_body(self, builder: RequestBuilder) -> Result<RequestBuilder>
    where
        Self: Sized,
    {
        self.body.configure(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum StatusCallbackEvent {
    Initiated,
    Ringing,
    Answered,
    Completed,
}

impl StatusCallbackEvent {
    fn serialize_initiated<S>(
        status_callback_event: &Option<bool>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(true) = status_callback_event {
            serializer.serialize_str("initiated")
        } else {
            serializer.serialize_none()
        }
    }

    fn serialize_ringing<S>(
        status_callback_event: &Option<bool>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(true) = status_callback_event {
            serializer.serialize_str("ringing")
        } else {
            serializer.serialize_none()
        }
    }

    fn serialize_answered<S>(
        status_callback_event: &Option<bool>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(true) = status_callback_event {
            serializer.serialize_str("answered")
        } else {
            serializer.serialize_none()
        }
    }

    fn serialize_completed<S>(
        status_callback_event: &Option<bool>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(true) = status_callback_event {
            serializer.serialize_str("completed")
        } else {
            serializer.serialize_none()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordingStatusCallbackEvent {
    InProgress,
    Completed,
    Absent,
}

impl RecordingStatusCallbackEvent {
    fn serialize_in_progress<S>(
        status_callback_event: &Option<bool>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(true) = status_callback_event {
            serializer.serialize_str("in-progress")
        } else {
            serializer.serialize_none()
        }
    }

    fn serialize_completed<S>(
        status_callback_event: &Option<bool>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(true) = status_callback_event {
            serializer.serialize_str("completed")
        } else {
            serializer.serialize_none()
        }
    }

    fn serialize_absent<S>(
        status_callback_event: &Option<bool>,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(true) = status_callback_event {
            serializer.serialize_str("absent")
        } else {
            serializer.serialize_none()
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatusCallbackParams {
    pub call_sid: String,
    pub account_sid: String,
    pub call_status: CallStatus,
    pub api_version: ApiVersion,
    pub direction: String,
    pub forwarded_from: Option<String>,
    pub from: String,
    pub to: String,
    pub caller_name: Option<String>,
    pub parent_call_sid: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatusCallbackEventParams {
    pub call_status: CallStatus,
    pub duration: Option<String>,
    pub call_duration: Option<String>,
    pub sip_response_code: Option<String>,
    pub recording_url: Option<String>,
    pub recording_sid: Option<String>,
    pub recording_duration: Option<String>,
    pub timestamps: Option<String>,
    pub callback_source: Option<String>,
    pub sequence_number: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordingStatusCallbackParams {
    pub account_sid: String,
    pub call_sid: String,
    pub recording_sid: String,
    pub recording_url: String,
    pub recording_status: RecordingStatus,
    pub recording_duration: Option<String>,
    pub recording_channels: Option<u32>,
    pub recording_time: Option<String>,
    pub recording_source: Option<String>,
    pub recording_track: Option<RecordingTrack>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordingStatus {
    InProgress,
    Completed,
    Absent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordingTrack {
    Inbound,
    Outbound,
    Both,
}

#[derive(Clone, Debug)]
pub struct FetchCall {
    pub account_sid: String,
    pub call_sid: String,
}

impl FetchCall {
    pub fn new(account_sid: impl Into<String>, call_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
            call_sid: call_sid.into(),
        }
    }
}

impl TwilioEndpoint for FetchCall {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = CallResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{CallSid}", &self.call_sid),
        ]
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

impl ByToAndFrom for ListCalls {}
impl CallQueryMarker for ListCalls {}

#[derive(Clone, Debug)]
pub struct ListCalls {
    pub account_sid: String,
    pub query: TwilioQuery<Self>,
}

impl ListCalls {
    pub fn new(account_sid: impl Into<String>, query: TwilioQuery<Self>) -> Self {
        Self {
            account_sid: account_sid.into(),
            query,
        }
    }
}

impl TwilioEndpoint for ListCalls {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Calls.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ListCallsResponse;

    fn query_params(&self) -> Option<QueryValues> {
        Some(self.query.params.clone())
    }

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{AccountSid}", &self.account_sid)]
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListCallsResponse {
    pub calls: Vec<CallResponse>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct UpdateCall<'a> {
    pub account_sid: String,
    pub call_sid: String,
    pub body: RequestBody<UpdateCallBody<'a>>,
}

impl<'a> UpdateCall<'a> {
    pub fn new(
        account_sid: impl Into<String>,
        call_sid: impl Into<String>,
        body: UpdateCallBody<'a>,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            call_sid: call_sid.into(),
            body: RequestBody::Form(body),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateCallBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateCallStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twiml: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<u32>,
}

impl<'a> UpdateCallBody<'a> {
    pub fn twiml(twiml: &'a str) -> Self {
        Self {
            twiml: Some(twiml),
            ..Default::default()
        }
    }

    pub fn url(url: &'a str) -> Self {
        Self {
            url: Some(url),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateCallStatus {
    Canceled,
    Completed,
}

impl TwilioEndpoint for UpdateCall<'_> {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = CallResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.call_sid),
        ]
    }

    fn configure_request_body(self, builder: RequestBuilder) -> Result<RequestBuilder>
    where
        Self: Sized,
    {
        self.body.configure(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug)]
pub struct DeleteCall {
    pub account_sid: String,
    pub call_sid: String,
}

impl DeleteCall {
    pub fn new(account_sid: impl Into<String>, call_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
            call_sid: call_sid.into(),
        }
    }
}

impl TwilioEndpoint for DeleteCall {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json";

    const METHOD: Method = Method::DELETE;

    type ResponseBody = ();

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.call_sid),
        ]
    }

    async fn response_body(_resp: Response) -> Result<Self::ResponseBody> {
        Ok(())
    }
}
