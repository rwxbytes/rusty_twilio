//! Conference endpoints

use super::*;
use crate::endpoints::applications::ApiVersion;
use crate::url::query::{ByDateCreatedAndDateUpdated, ByFriendlyName, ConferenceQueryMarker};
use crate::TwilioQuery;

const ANNOUNCE_URL: &str = "AnnounceUrl";
const ANNOUNCE_METHOD: &str = "AnnounceMethod";
const BEEP: &str = "Beep";
const END_CONFERENCE_ON_EXIT: &str = "EndConferenceOnExit";
const START_CONFERENCE_ON_ENTER: &str = "StartConferenceOnEnter";
const WAIT_URL: &str = "WaitUrl";

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

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
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

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListConferencesResponse {
    pub conferences: Vec<ConferenceResponse>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Clone, Debug)]
/// See [Update A Conference Resource](https://www.twilio.com/docs/voice/api/conference-resource#update-a-conference-resource)
pub struct UpdateConference {
    pub account_sid: String,
    pub conference_sid: String,
    pub body: UpdateConferenceBody,
}

impl UpdateConference {
    pub fn new(
        account_sid: impl Into<String>,
        conference_sid: impl Into<String>,
        body: UpdateConferenceBody,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            body,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UpdateConferenceBody {
    pub params: Vec<(&'static str, String)>,
}

impl TwilioEndpoint for UpdateConference {
    const PATH: &'static str = "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = ConferenceResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{ConferenceSid}", &self.conference_sid),
        ]
    }

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Form(self.body.params.clone()))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

impl UpdateConferenceBody {
    pub fn with_conference_status(mut self, status: impl Into<String>) -> Self {
        self.params.push((STATUS, status.into()));
        self
    }

    pub fn with_announce_url(mut self, announce_url: impl Into<String>) -> Self {
        self.params.push((ANNOUNCE_URL, announce_url.into()));
        self
    }

    pub fn with_announce_method(mut self, announce_method: impl Into<String>) -> Self {
        self.params.push((ANNOUNCE_METHOD, announce_method.into()));
        self
    }
}

#[derive(Clone, Debug)]
pub struct CreateParticipant {
    pub account_sid: String,
    pub conference_sid: String,
    pub body: CreateParticipantBody,
}

#[derive(Clone, Debug)]
pub struct CreateParticipantBody {
    pub params: Vec<(&'static str, String)>,
}

impl CreateParticipant {
    pub fn new(
        account_sid: impl Into<String>,
        conference_sid: impl Into<String>,
        body: CreateParticipantBody,
    ) -> Self {
        Self {
            account_sid: account_sid.into(),
            conference_sid: conference_sid.into(),
            body,
        }
    }
}

impl CreateParticipantBody {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            params: vec![(FROM, from.into()), (TO, to.into())],
        }
    }
    pub fn with_start_conference_on_enter(mut self, start_conference_on_enter: bool) -> Self {
        self.params.push((
            START_CONFERENCE_ON_ENTER,
            start_conference_on_enter.to_string(),
        ));
        self
    }

    pub fn with_end_conference_on_exit(mut self, end_conference_on_exit: bool) -> Self {
        self.params
            .push((END_CONFERENCE_ON_EXIT, end_conference_on_exit.to_string()));
        self
    }

    pub fn with_beep(mut self, beep: bool) -> Self {
        self.params.push((BEEP, beep.to_string()));
        self
    }

    pub fn with_wait_url(mut self, wait_url: impl Into<String>) -> Self {
        self.params.push((WAIT_URL, wait_url.into()));
        self
    }
}

impl TwilioEndpoint for CreateParticipant {
    const PATH: &'static str =
        "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json";
    const METHOD: Method = Method::POST;

    //type ResponseBody = CreateParticipantResponse;
    type ResponseBody = serde_json::Value;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("{AccountSid}", &self.account_sid),
            ("{ConferenceSid}", &self.conference_sid),
        ]
    }

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Form(self.body.params.clone()))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}
