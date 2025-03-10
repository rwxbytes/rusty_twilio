//! Call endpoints
//! See [Call Resource reference](https://www.twilio.com/docs/voice/api/call-resource)

use super::*;
use crate::endpoints::applications::ApiVersion;
use crate::url::query::{ByToAndFrom, CallQueryMarker, TwilioQuery};
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
    // TODO: SIP & other fields based on Twiml verbs
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

#[derive(Clone, Debug, Deserialize, Display)]
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

#[derive(Clone, Debug)]
pub struct CreateCall {
    pub account_sid: String,
    pub body: CreateCallBody,
}

#[derive(Clone, Debug, Default)]
pub struct CreateCallBody {
    pub params: Vec<(&'static str, String)>,
}

impl CreateCall {
    pub fn new(account_sid: impl Into<String>, body: CreateCallBody) -> Self {
        Self {
            account_sid: account_sid.into(),
            body,
        }
    }
}

impl TwilioEndpoint for CreateCall {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Calls.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = CallResponse;

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
pub enum TwimlSrc {
    Url(String),
    Twiml(String),
    ApplicationSid(String),
}

impl TwimlSrc {
    pub fn url(url: impl Into<String>) -> Self {
        TwimlSrc::Url(url.into())
    }

    pub fn twiml(twiml: impl Into<String>) -> Self {
        TwimlSrc::Twiml(twiml.into())
    }

    pub fn application_sid(application_sid: impl Into<String>) -> Self {
        TwimlSrc::ApplicationSid(application_sid.into())
    }

    pub fn to_string(&self) -> String {
        match self {
            TwimlSrc::Url(url) => url.clone(),
            TwimlSrc::Twiml(twiml) => twiml.clone(),
            TwimlSrc::ApplicationSid(application_sid) => application_sid.clone(),
        }
    }
}

impl AsRef<str> for TwimlSrc {
    fn as_ref(&self) -> &str {
        match self {
            TwimlSrc::Url(s) => s.as_ref(),
            TwimlSrc::Twiml(s) => s.as_ref(),
            TwimlSrc::ApplicationSid(s) => s.as_ref(),
        }
    }
}

impl From<TwimlSrc> for (&'static str, String) {
    fn from(src: TwimlSrc) -> Self {
        match src {
            TwimlSrc::Url(url) => (URL, url),
            TwimlSrc::Twiml(twiml) => (TWIML, twiml),
            TwimlSrc::ApplicationSid(sid) => (APPLICATION_SID, sid),
        }
    }
}

#[derive(Clone, Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum StatusCallbackEvent {
    Initiated,
    Ringing,
    Answered,
    Completed,
}

#[derive(Clone, Debug, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum RecordingStatusCallbackEvent {
    InProgress,
    Completed,
    Absent,
}

impl CreateCallBody {
    pub fn new(to: impl Into<String>, from: impl Into<String>, twilml_src: TwimlSrc) -> Self {
        let mut params = Vec::new();
        params.push((TO, to.into()));
        params.push((FROM, from.into()));
        params.push(twilml_src.into());
        Self { params }
    }

    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.params.push((METHOD, method.into()));
        self
    }

    pub fn with_fallback_url(mut self, fallback_url: impl Into<String>) -> Self {
        self.params.push((FALLBACK_URL, fallback_url.into()));
        self
    }

    pub fn with_fallback_method(mut self, fallback_method: impl Into<String>) -> Self {
        self.params.push((FALLBACK_METHOD, fallback_method.into()));
        self
    }

    pub fn with_status_callback(mut self, status_callback: impl Into<String>) -> Self {
        self.params.push((STATUS_CALLBACK, status_callback.into()));
        self
    }

    pub fn with_status_callback_method(
        mut self,
        status_callback_method: impl Into<String>,
    ) -> Self {
        self.params
            .push((STATUS_CALLBACK_METHOD, status_callback_method.into()));
        self
    }

    /// The call progress events that we will send to the status_callback URL.
    /// Can be: initiated, ringing, answered, and completed.
    /// If no event is specified, we send the completed status.
    /// If you want to receive multiple events, specify each one in a separate status_callback_event parameter
    pub fn with_status_callback_event(
        mut self,
        status_callback_event: StatusCallbackEvent,
    ) -> Self {
        self.params
            .push((STATUS_CALLBACK_EVENT, status_callback_event.to_string()));
        self
    }

    pub fn with_send_digits(mut self, send_digits: impl Into<String>) -> Self {
        self.params.push((SEND_DIGITS, send_digits.into()));
        self
    }

    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.params.push((TIMEOUT, timeout.to_string()));
        self
    }

    pub fn with_record(mut self, record: bool) -> Self {
        self.params.push((RECORD, record.to_string()));
        self
    }

    pub fn with_recording_channels(mut self, recording_channels: impl Into<String>) -> Self {
        self.params
            .push((RECORDING_CHANNELS, recording_channels.into()));
        self
    }

    pub fn with_recording_status_callback(
        mut self,
        recording_status_callback: impl Into<String>,
    ) -> Self {
        self.params
            .push((RECORDING_STATUS_CALLBACK, recording_status_callback.into()));
        self
    }

    pub fn with_recording_status_callback_method(
        mut self,
        recording_status_callback_method: impl Into<String>,
    ) -> Self {
        self.params.push((
            RECORDING_STATUS_CALLBACK_METHOD,
            recording_status_callback_method.into(),
        ));
        self
    }

    pub fn with_recording_status_callback_event(
        mut self,
        recording_status_callback_event: RecordingStatusCallbackEvent,
    ) -> Self {
        self.params.push((
            RECORDING_STATUS_CALLBACK_EVENT,
            recording_status_callback_event.to_string(),
        ));
        self
    }

    pub fn with_recording_track(mut self, recording_track: impl Into<String>) -> Self {
        self.params.push((RECORDING_TRACK, recording_track.into()));
        self
    }

    pub fn with_sip_auth_username(mut self, sip_auth_username: impl Into<String>) -> Self {
        self.params
            .push((SIP_AUTH_USERNAME, sip_auth_username.into()));
        self
    }

    pub fn with_sip_auth_password(mut self, sip_auth_password: impl Into<String>) -> Self {
        self.params
            .push((SIP_AUTH_PASSWORD, sip_auth_password.into()));
        self
    }

    pub fn with_machine_detection(mut self, machine_detection: impl Into<String>) -> Self {
        self.params
            .push((MACHINE_DETECTION, machine_detection.into()));
        self
    }

    pub fn with_machine_detection_timeout(mut self, machine_detection_timeout: u32) -> Self {
        self.params.push((
            MACHINE_DETECTION_TIMEOUT,
            machine_detection_timeout.to_string(),
        ));
        self
    }

    pub fn with_machine_detection_speech_threshold(
        mut self,
        machine_detection_speech_threshold: f32,
    ) -> Self {
        self.params.push((
            MACHINE_DETECTION_SPEECH_THRESHOLD,
            machine_detection_speech_threshold.to_string(),
        ));
        self
    }

    pub fn with_machine_detection_speech_end_threshold(
        mut self,
        machine_detection_speech_end_threshold: f32,
    ) -> Self {
        self.params.push((
            MACHINE_DETECTION_SPEECH_END_THRESHOLD,
            machine_detection_speech_end_threshold.to_string(),
        ));
        self
    }

    pub fn with_machine_detection_silence_timeout(
        mut self,
        machine_detection_silence_timeout: u32,
    ) -> Self {
        self.params.push((
            MACHINE_DETECTION_SILENCE_TIMEOUT,
            machine_detection_silence_timeout.to_string(),
        ));
        self
    }

    pub fn with_trim(mut self, trim: impl Into<String>) -> Self {
        self.params.push((TRIM, trim.into()));
        self
    }

    pub fn with_caller_id(mut self, caller_id: impl Into<String>) -> Self {
        self.params.push((CALLER_ID, caller_id.into()));
        self
    }

    pub fn with_async_amd(mut self, async_amd: bool) -> Self {
        self.params.push((ASYNC_AMD, async_amd.to_string()));
        self
    }

    pub fn with_async_amd_status_callback(
        mut self,
        async_amd_status_callback: impl Into<String>,
    ) -> Self {
        self.params
            .push((ASYNC_AMD_STATUS_CALLBACK, async_amd_status_callback.into()));
        self
    }

    pub fn with_async_amd_status_callback_method(
        mut self,
        async_amd_status_callback_method: impl Into<String>,
    ) -> Self {
        self.params.push((
            ASYNC_AMD_STATUS_CALLBACK_METHOD,
            async_amd_status_callback_method.into(),
        ));
        self
    }

    pub fn with_byoc_sid(mut self, byoc_sid: impl Into<String>) -> Self {
        self.params.push((BYOC_SID, byoc_sid.into()));
        self
    }

    pub fn with_call_reason(mut self, call_reason: impl Into<String>) -> Self {
        self.params.push((CALL_REASON, call_reason.into()));
        self
    }

    pub fn with_call_token(mut self, call_token: impl Into<String>) -> Self {
        self.params.push((CALL_TOKEN, call_token.into()));
        self
    }

    pub fn with_time_limit(mut self, time_limit: u32) -> Self {
        self.params.push((TIME_LIMIT, time_limit.to_string()));
        self
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

#[derive(Clone, Debug, Deserialize)]
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

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
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

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListCallsResponse {
    pub calls: Vec<CallResponse>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Clone, Debug)]
pub struct UpdateCall {
    pub account_sid: String,
    pub call_sid: String,
    pub body: UpdateCallBody,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateCallBody {
    pub params: Vec<(&'static str, String)>,
}

#[derive(Clone, Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum UpdateCallStatus {
    Canceled,
    Completed,
}

impl UpdateCall {
    pub fn new(
        account_sid: impl Into<String>,
        call_sid: impl Into<String>,
        body: UpdateCallBody,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            call_sid: call_sid.into(),
            body,
        }
    }
}

// TODO: share methods
impl UpdateCallBody {
    pub fn with_status(mut self, status: UpdateCallStatus) -> Self {
        self.params.push((STATUS, status.to_string()));
        self
    }

    pub fn with_twiml(mut self, twiml: impl Into<String>) -> Self {
        self.params.push((TWIML, twiml.into()));
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.params.push((URL, url.into()));
        self
    }

    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.params.push((METHOD, method.into()));
        self
    }

    pub fn with_fallback_url(mut self, fallback_url: impl Into<String>) -> Self {
        self.params.push((FALLBACK_URL, fallback_url.into()));
        self
    }

    pub fn with_fallback_method(mut self, fallback_method: impl Into<String>) -> Self {
        self.params.push((FALLBACK_METHOD, fallback_method.into()));
        self
    }

    pub fn with_status_callback(mut self, status_callback: impl Into<String>) -> Self {
        self.params.push((STATUS_CALLBACK, status_callback.into()));
        self
    }

    pub fn with_status_callback_method(
        mut self,
        status_callback_method: impl Into<String>,
    ) -> Self {
        self.params
            .push((STATUS_CALLBACK_METHOD, status_callback_method.into()));
        self
    }

    pub fn with_time_limit(mut self, time_limit: u32) -> Self {
        self.params.push((TIME_LIMIT, time_limit.to_string()));
        self
    }
}

impl TwilioEndpoint for UpdateCall {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = CallResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.call_sid),
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

    async fn response_body(self, _resp: Response) -> Result<Self::ResponseBody> {
        Ok(())
    }
}
