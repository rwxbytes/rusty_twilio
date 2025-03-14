//! Conference endpoints
use super::*;

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

const START_CONFERENCE_ON_ENTER: &str = "startConferenceOnEnter";
const END_CONFERENCE_ON_EXIT: &str = "endConferenceOnExit";

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
