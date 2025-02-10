pub mod accounts;
pub mod applications;
pub mod voice;

pub use crate::Result;
use std::collections::HashMap;

use reqwest::{Method, Response, Url};
pub use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum RequestBody {
    Empty,
    Json(serde_json::Value),
    Multipart(reqwest::multipart::Form),
    Form(HashMap<String, String>),
}

pub(crate) type QueryValues = Vec<(&'static str, String)>;

#[derive(Clone, Debug, Default)]
pub enum Region {
    Australia,
    Ireland,
    #[default]
    UnitedStates,
}

#[allow(async_fn_in_trait)]
pub trait TwilioEndpoint {
    const BASE_URL: &'static str = "https://api.twilio.com";

    const PATH: &'static str;

    const METHOD: Method;

    type ResponseBody;

    fn query_params(&self) -> Option<QueryValues> {
        None
    }

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![]
    }

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Empty)
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody>;

    fn url(&self) -> Url {
        let mut url = Self::BASE_URL.parse::<Url>().unwrap();

        let mut path = Self::PATH.to_string();

        for (placeholder, id) in self.path_params() {
            path = path.replace(placeholder, id);
        }

        url.set_path(&path);

        if let Some(query_params) = self.query_params() {
            let query_string = query_params
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");

            url.set_query(Some(&query_string))
        }

        url
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
    pub first_page_uri: String,
    pub end: usize,
    pub start: usize,
    pub uri: String,
    pub next_page_uri: Option<String>,
    pub previous_page_uri: Option<String>,
}

const API_VERSION: &str = "ApiVersion";
const VOICE_URL: &str = "VoiceUrl";
const VOICE_METHOD: &str = "VoiceMethod";
const VOICE_FALLBACK_URL: &str = "VoiceFallbackUrl";
const VOICE_FALLBACK_METHOD: &str = "VoiceFallbackMethod";
const STATUS_CALLBACK: &str = "StatusCallback";
const STATUS_CALLBACK_METHOD: &str = "StatusCallbackMethod";
const SMS_URL: &str = "SmsUrl";
const SMS_METHOD: &str = "SmsMethod";
const SMS_FALLBACK_URL: &str = "SmsFallbackUrl";
const SMS_FALLBACK_METHOD: &str = "SmsFallbackMethod";
const MESSAGE_STATUS_CALLBACK: &str = "MessageStatusCallback";
const FRIENDLY_NAME: &str = "FriendlyName";
const PUBLIC_APPLICATION_CONNECT_ENABLED: &str = "PublicApplicationConnectEnabled";
const TO: &str = "To";
const FROM: &str = "From";
const URL: &str = "Url";
const APPLICATION_SID: &str = "ApplicationSid";
const TWIML: &str = "Twiml";
const METHOD: &str = "Method";
const FALLBACK_URL: &str = "FallbackUrl";
const FALLBACK_METHOD: &str = "FallbackMethod";
const STATUS_CALLBACK_EVENT: &str = "StatusCallbackEvent";
const SEND_DIGITS: &str = "SendDigits";
const TIMEOUT: &str = "Timeout";
const RECORD: &str = "Record";
const RECORDING_CHANNELS: &str = "RecordingChannels";
const RECORDING_STATUS_CALLBACK: &str = "RecordingStatusCallback";
const RECORDING_STATUS_CALLBACK_METHOD: &str = "RecordingStatusCallbackMethod";
const RECORDING_STATUS_CALLBACK_EVENT: &str = "RecordingStatusCallbackEvent";
const RECORDING_TRACK: &str = "RecordingTrack";
const SIP_AUTH_USERNAME: &str = "SipAuthUsername";
const SIP_AUTH_PASSWORD: &str = "SipAuthPassword";
const MACHINE_DETECTION: &str = "MachineDetection";
const MACHINE_DETECTION_TIMEOUT: &str = "MachineDetectionTimeout";
const MACHINE_DETECTION_SILENCE_TIMEOUT: &str = "MachineDetectionSilenceTimeout";
const MACHINE_DETECTION_SPEECH_THRESHOLD: &str = "MachineDetectionSpeechThreshold";
const MACHINE_DETECTION_SPEECH_END_THRESHOLD: &str = "MachineDetectionSpeechEndThreshold";
const TRIM: &str = "Trim";
const ASYNC_AMD: &str = "AsyncAmd";
const ASYNC_AMD_STATUS_CALLBACK: &str = "AsyncAmdStatusCallback";
const ASYNC_AMD_STATUS_CALLBACK_METHOD: &str = "AsyncAmdStatusCallbackMethod";
const CALLER_ID: &str = "CallerId";
const BYOC_SID: &str = "ByocSid";
const CALL_REASON: &str = "CallReason";
const CALL_TOKEN: &str = "CallToken";
const TIME_LIMIT: &str = "TimeLimit";
