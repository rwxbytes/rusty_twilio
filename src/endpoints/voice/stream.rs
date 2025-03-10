//! Stream endpoints
//! See [Twilio Stream API](https://www.twilio.com/docs/voice/api/stream-resource)

use super::*;
use crate::error::TwilioError;

#[derive(Clone, Debug, Deserialize)]
pub struct StreamResponse {
    /// The SID of the Stream resource.
    pub sid: String,
    /// The SID of the Account that created this Stream resource.
    pub account_sid: String,
    /// The SID of the Call the Stream resource is associated with.
    pub call_sid: String,
    /// The user-specified name of this Stream, if one was given when the Stream was created. This can be used to stop the Stream.
    pub name: Option<String>,
    /// The status of the Stream. Can be in-progress or stopped.
    pub status: StreamStatus,
    /// The date and time in GMT when the Stream resource was created specified in RFC 2822 format.
    pub date_updated: Option<String>,
    /// The URI for this resource, relative to https://api.twilio.com.
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StreamStatus {
    InProgress,
    Stopped,
}

#[derive(Clone, Debug)]
pub struct CreateStream {
    pub account_sid: String,
    pub call_sid: String,
    pub body: CreateStreamBody,
}

#[derive(Clone, Debug)]
pub struct CreateStreamBody {
    pub params: Vec<(&'static str, String)>,
}

impl CreateStream {
    pub fn new(
        account_sid: impl Into<String>,
        call_sid: impl Into<String>,
        body: CreateStreamBody,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            call_sid: call_sid.into(),
            body,
        }
    }
}

impl CreateStreamBody {
    pub fn new(url: String) -> Self {
        Self {
            params: vec![(URL, url)],
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.params.push((NAME, name.into()));
        self
    }

    pub fn with_track(mut self, track: impl Into<String>) -> Self {
        self.params.push((TRACK, track.into()));
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
    /// See [Custom Parameters](https://www.twilio.com/docs/voice/api/stream-resource#custom-parameters)
    // TODO: may need to change type again
    pub fn with_custom_parameter(
        mut self,
        key: &'static str,
        value: impl Into<String>,
    ) -> Self {
        self.params.push((key, value.into()));
        self
    }
}

impl TwilioEndpoint for CreateStream {
    const PATH: &'static str = "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = StreamResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{CallSid}", &self.call_sid),
        ]
    }

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Form(self.body.params.clone()))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

/// See [Update a Stream](https://www.twilio.com/docs/voice/api/stream-resource#update-a-stream-resource)
///
/// To stop a live unidirectional Media Stream, update the Stream resource's status to stopped.
#[derive(Clone, Debug)]
pub struct UpdateStream {
    pub account_sid: String,
    pub call_sid: String,
    pub stream_sid: String,
}

impl UpdateStream {
    pub fn new(
        account_sid: impl Into<String>,
        call_sid: impl Into<String>,
        stream_sid: impl Into<String>,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            call_sid: call_sid.into(),
            stream_sid: stream_sid.into(),
        }
    }
}

impl TwilioEndpoint for UpdateStream {
    const PATH: &'static str =
        "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams/{Sid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = StreamResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{CallSid}", &self.call_sid),
            ("{Sid}", &self.stream_sid),
        ]
    }

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Form(vec![(STATUS, "stopped".to_string())]))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

/// See [Websocket Messages From Twilio](https://www.twilio.com/docs/voice/media-streams/websocket-messages#websocket-messages-from-twilio)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TwilioMessage {
    Connected(ConnectedMessage),
    Start(StartMessage),
    Media(MediaMessage),
    Mark(MarkMessage),
    Stop(StopMessage),
    Dtmf(DtmfMessage),
}

impl TryFrom<&str> for TwilioMessage {
    type Error = TwilioError;

    fn try_from(value: &str) -> Result<Self> {
        Ok(serde_json::from_str(value)?)
    }
}

/// See [Connected Message](https://www.twilio.com/docs/voice/media-streams/websocket-messages#connected-message)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedMessage {
    pub event: String,
    pub protocol: String,
    pub version: String,
}

/// See [Start Message](https://www.twilio.com/docs/voice/media-streams/websocket-messages#start-message)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartMessage {
    pub event: String,
    pub sequence_number: String,
    pub start: StartMetadata,
    pub stream_sid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartMetadata {
    pub stream_sid: String,
    pub account_sid: String,
    pub call_sid: String,
    pub tracks: Vec<Track>,
    pub custom_parameters: serde_json::Value,
    pub media_format: MediaFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaFormat {
    pub encoding: String,
    pub sample_rate: u32,
    pub channels: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Track {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
}

/// See [Media Message](https://www.twilio.com/docs/voice/media-streams/websocket-messages#media-message)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaMessage {
    pub event: String,
    pub stream_sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    pub media: Media,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<Track>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl MediaMessage {
    pub fn new(stream_sid: impl Into<String>, payload: impl Into<String>) -> Self {
        MediaMessage {
            event: "media".to_string(),
            stream_sid: stream_sid.into(),
            sequence_number: None,
            media: Media {
                payload: payload.into(),
                ..Default::default()
            },
        }
    }
}

/// See [Stop Message](https://www.twilio.com/docs/voice/media-streams/websocket-messages#stop-message)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopMessage {
    pub event: String,
    pub stream_sid: String,
    pub sequence_number: String,
    pub stop: Stop,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub account_sid: String,
    pub call_sid: String,
}

/// See [DTMF Message](https://www.twilio.com/docs/voice/media-streams/websocket-messages#dtmf-message)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DtmfMessage {
    pub event: String,
    pub stream_sid: String,
    pub sequence_number: String,
    pub dtmf: Dtmf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dtmf {
    pub digit: String,
    pub track: String,
}

/// See [Mark Message](https://www.twilio.com/docs/voice/media-streams/websocket-messages#mark-message)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkMessage {
    pub event: String,
    pub stream_sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    pub mark: Mark,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mark {
    pub name: String,
}

/// [Sending Clear Messages](https://www.twilio.com/docs/voice/media-streams/websocket-messages#send-a-clear-message)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearMessage {
    pub event: String,
    pub stream_sid: String,
}

impl ClearMessage {
    pub fn new(sid: &str) -> Self {
        ClearMessage {
            event: "clear".to_string(),
            stream_sid: sid.to_string(),
        }
    }
}
