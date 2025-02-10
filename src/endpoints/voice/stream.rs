//! Stream endpoints
//! See [Twilio Stream API](https://www.twilio.com/docs/voice/api/stream-resource)
use super::*;

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
    pub params: HashMap<String, String>,
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
        let mut params = HashMap::new();
        params.insert(URL.to_string(), url);
        Self { params }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.params.insert(NAME.to_string(), name.into());
        self
    }

    pub fn with_track(mut self, track: impl Into<String>) -> Self {
        self.params.insert(TRACK.to_string(), track.into());
        self
    }

    pub fn with_status_callback(mut self, status_callback: impl Into<String>) -> Self {
        self.params
            .insert(STATUS_CALLBACK.to_string(), status_callback.into());
        self
    }

    pub fn with_status_callback_method(
        mut self,
        status_callback_method: impl Into<String>,
    ) -> Self {
        self.params.insert(
            STATUS_CALLBACK_METHOD.to_string(),
            status_callback_method.into(),
        );
        self
    }

    /// See [Custom Parameters](https://www.twilio.com/docs/voice/api/stream-resource#custom-parameters)
    pub fn with_custom_parameter(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.params.insert(key.into(), value.into());
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
        let mut form = HashMap::new();
        form.insert(STATUS.to_string(), "stopped".to_string());
        Ok(RequestBody::Form(form))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}
