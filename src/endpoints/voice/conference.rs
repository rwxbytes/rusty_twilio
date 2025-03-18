//! Conference endpoints

use super::*;
use crate::endpoints::applications::ApiVersion;
use crate::endpoints::voice::call::RecordingTrack;
use crate::url::query::{
    ByDateCreatedAndDateUpdated, ByFriendlyName, ConferenceQueryMarker, ParticipantQueryMarker,
};
use crate::TwilioQuery;
use reqwest::RequestBuilder;

#[derive(Clone, Debug, Deserialize)]
/// See [Conference Properties](https://www.twilio.com/docs/voice/api/conference-resource#conference-properties)
pub struct ConferenceResponse {
    pub account_sid: String,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub api_version: ApiVersion,
    pub friendly_name: Option<String>,
    pub region: Option<String>,
    pub sid: String,
    pub status: String,
    pub uri: String,
    pub subresource_uris: Option<serde_json::Value>,
    pub reason_conference_ended: Option<String>,
    pub call_sid_ending_conference: Option<String>,
}

#[derive(Clone, Debug)]
/// See [Fetch Conference](https://www.twilio.com/docs/voice/api/conference-resource#fetch-a-conference-resource)
pub struct FetchConference {
    pub account_sid: String,
    pub conference_sid: String,
}

impl FetchConference {
    pub fn new(account_sid: impl Into<String>, conference_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
        }
    }
}

impl TwilioEndpoint for FetchConference {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ConferenceResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.conference_sid),
        ]
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

impl ByFriendlyName for ListConferences {}
impl ByDateCreatedAndDateUpdated for ListConferences {}
impl ConferenceQueryMarker for ListConferences {}

#[derive(Clone, Debug)]
/// See [Read Multiple Conference Resources](https://www.twilio.com/docs/voice/api/conference-resource#read-multiple-conference-resources)
pub struct ListConferences {
    pub account_sid: String,
    pub query: Option<TwilioQuery<Self>>,
}

impl ListConferences {
    pub fn new(account_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
            query: None,
        }
    }

    pub fn with_query(mut self, query: TwilioQuery<Self>) -> Self {
        self.query = Some(query);
        self
    }
}

impl TwilioEndpoint for ListConferences {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Conferences.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ListConferencesResponse;

    fn query_params(&self) -> Option<QueryValues> {
        self.query.as_ref().map(|q| q.params.clone())
    }

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{AccountSid}", &self.account_sid)]
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListConferencesResponse {
    pub conferences: Vec<ConferenceResponse>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug)]
/// See [Update A Conference Resource](https://www.twilio.com/docs/voice/api/conference-resource#update-a-conference-resource)
pub struct UpdateConference<'a> {
    pub account_sid: String,
    pub conference_sid: String,
    pub body: RequestBody<UpdateConferenceBody<'a>>,
}

impl<'a> UpdateConference<'a> {
    pub fn new(
        account_sid: impl Into<String>,
        conference_sid: impl Into<String>,
        body: UpdateConferenceBody<'a>,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            body: RequestBody::Form(body),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateConferenceBody<'a> {
    pub status: Option<&'a str>,
    pub announce_url: Option<&'a str>,
    pub announce_method: Option<&'a str>,
}

impl TwilioEndpoint for UpdateConference<'_> {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = ConferenceResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{Sid}", &self.conference_sid),
        ]
    }

    fn configure_request(self, builder: RequestBuilder) -> Result<RequestBuilder>
    where
        Self: Sized,
    {
        self.body.configure(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
/// See [Participant Properties](https://www.twilio.com/docs/voice/api/conference-participant-resource#participant-properties)
pub struct ParticipantResponse {
    pub account_sid: String,
    pub conference_sid: String,
    pub call_sid: String,
    pub label: Option<String>,
    pub call_sid_to_coach: Option<String>,
    pub coaching: Option<bool>,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub end_conference_on_exit: Option<bool>,
    pub muted: Option<bool>,
    pub hold: Option<bool>,
    pub start_conference_on_enter: Option<bool>,
    pub status: Option<String>,
    pub queue_time: Option<String>,
    pub uri: String,
}

#[derive(Debug)]
pub struct CreateParticipant<'a> {
    pub account_sid: String,
    pub conference_sid: String,
    pub body: RequestBody<CreateParticipantBody<'a>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
// TODO: flatten some sections like machine detection
pub struct CreateParticipantBody<'a> {
    pub from: &'a str,
    pub to: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "CreateParticipantBody::join_events")]
    pub status_callback_event: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beep: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_conference_on_enter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_conference_on_exit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_record: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_trim: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_status_callback: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "CreateParticipantBody::join_events")]
    pub conference_status_callback_event: Vec<&'a str>,
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
    pub region: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_recording_status_callback: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_recording_status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "CreateParticipantBody::join_events")]
    pub recording_status_callback_event: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conference_recording_status_callback_event: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coaching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_sid_to_coach: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter_buffer_size: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_reason: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_track: Option<RecordingTrack>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timelimit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_speech_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_end_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection_silence_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amd_status_callback: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amd_status_callback_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_token: Option<&'a str>,
}

impl<'a> CreateParticipantBody<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Self {
        Self {
            from,
            to,
            ..Default::default()
        }
    }
    fn join_events<S>(events: &Vec<&'a str>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if events.is_empty() {
            serializer.serialize_none()
        } else {
            let joined = events.join(" ");
            serializer.serialize_str(&joined)
        }
    }
}

impl<'a> CreateParticipant<'a> {
    pub fn new(
        account_sid: impl Into<String>,
        conference_sid: impl Into<String>,
        body: CreateParticipantBody<'a>,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            body: RequestBody::Form(body),
        }
    }
}

impl TwilioEndpoint for CreateParticipant<'_> {
    const PATH: &'static str =
        "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json";
    const METHOD: Method = Method::POST;

    type ResponseBody = ParticipantResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{ConferenceSid}", &self.conference_sid),
        ]
    }

    fn configure_request(self, builder: RequestBuilder) -> Result<RequestBuilder> {
        self.body.configure(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Debug)]
/// See [Fetch A Participant Resource](https://www.twilio.com/docs/voice/api/conference-participant-resource#fetch-a-participant-resource)
pub struct FetchParticipant {
    pub account_sid: String,
    pub conference_sid: String,
    pub participant_sid: String,
}

impl FetchParticipant {
    pub fn new<T>(account_sid: T, conference_sid: T, participant_sid: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            participant_sid: participant_sid.into(),
        }
    }
}

impl TwilioEndpoint for FetchParticipant {
    const PATH: &'static str =
        "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ParticipantResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{ConferenceSid}", &self.conference_sid),
            ("{CallSid}", &self.participant_sid),
        ]
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

impl ParticipantQueryMarker for ListParticipants {}

#[derive(Debug)]
/// See [Read Multiple Participant Resources](https://www.twilio.com/docs/voice/api/conference-participant-resource#read-multiple-participant-resources)
pub struct ListParticipants {
    pub account_sid: String,
    pub conference_sid: String,
    pub query: Option<TwilioQuery<Self>>,
}

impl ListParticipants {
    pub fn new(account_sid: impl Into<String>, conference_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            query: None,
        }
    }

    pub fn with_query(mut self, query: TwilioQuery<Self>) -> Self {
        self.query = Some(query);
        self
    }
}

impl TwilioEndpoint for ListParticipants {
    const PATH: &'static str =
        "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ListParticipantsResponse;

    fn query_params(&self) -> Option<QueryValues> {
        self.query.as_ref().map(|q| q.params.clone())
    }

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{ConferenceSid}", &self.conference_sid),
        ]
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListParticipantsResponse {
    pub participants: Vec<ParticipantResponse>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug)]
/// See [Update A Participant Resource](https://www.twilio.com/docs/voice/api/conference-participant-resource#update-a-participant-resource)
pub struct UpdateParticipant<'a> {
    pub account_sid: String,
    pub conference_sid: String,
    pub participant_sid: String,
    pub body: RequestBody<UpdateParticipantBody<'a>>,
}

impl<'a> UpdateParticipant<'a> {
    pub fn new<T>(
        account_sid: T,
        conference_sid: T,
        participant_sid: T,
        body: UpdateParticipantBody<'a>,
    ) -> Self
    where
        T: Into<String>,
    {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            participant_sid: participant_sid.into(),
            body: RequestBody::Form(body),
        }
    }
}

impl TwilioEndpoint for UpdateParticipant<'_> {
    const PATH: &'static str =
        "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = ParticipantResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{ConferenceSid}", &self.conference_sid),
            ("{CallSid}", &self.participant_sid),
        ]
    }

    fn configure_request(self, builder: RequestBuilder) -> Result<RequestBuilder> {
        self.body.configure(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateParticipantBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beep_on_exit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_conference_on_exit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coaching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_sid_to_coach: Option<&'a str>,
}

#[derive(Debug)]
/// See [Delete A Participant Resource](https://www.twilio.com/docs/voice/api/conference-participant-resource#delete-a-participant-resource)
pub struct DeleteParticipant {
    pub account_sid: String,
    pub conference_sid: String,
    pub participant_sid: String,
}

impl DeleteParticipant {
    pub fn new<T>(account_sid: T, conference_sid: T, participant_sid: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            participant_sid: participant_sid.into(),
        }
    }
}

impl TwilioEndpoint for DeleteParticipant {
    const PATH: &'static str =
        "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json";

    const METHOD: Method = Method::DELETE;

    type ResponseBody = ();

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{ConferenceSid}", &self.conference_sid),
            ("{CallSid}", &self.participant_sid),
        ]
    }

    async fn response_body(_: Response) -> Result<Self::ResponseBody> {
        Ok(())
    }
}
