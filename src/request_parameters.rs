use crate::endpoints::applications::ApiVersion;
use crate::endpoints::voice::call::CallStatus;
use crate::endpoints::Deserialize;
use std::collections::HashMap;

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

impl TwilioRequestParams {
    pub fn is_no_answer(&self) -> bool {
        self.call_status == CallStatus::NoAnswer
    }
    pub fn get_extra(&self, key: &str) -> Option<&String> {
        self.extra.get(key)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// See [Conference Request Parameters](https://www.twilio.com/docs/voice/twiml/conference#attributes-statuscallback-parameters)
pub struct ConferenceRequestParams {
    pub conference_sid: String,
    pub friendly_name: String,
    pub account_sid: String,
    pub sequence_number: u32,
    pub timestamp: String,
    pub status_callback_event: Option<ConferenceEvent>,
    pub call_sid: Option<String>,
    pub muted: Option<bool>,
    pub hold: Option<bool>,
    pub coaching: Option<bool>,
    pub end_conference_on_exit: Option<bool>,
    pub start_conference_on_enter: Option<bool>,
    pub call_sid_ending_conference: Option<String>,
    pub participant_label_ending_conference: Option<String>,
    pub reason: Option<String>,
    pub reason_announcement_failed: Option<String>,
    pub announce_url: Option<String>,
    pub participation_call_status: Option<String>,
    pub event_name: Option<String>,
    pub recording_url: Option<String>,
    pub duration: Option<u32>,
    pub recording_file_size: Option<u32>,
}

impl ConferenceRequestParams {
    pub fn is_conference_end(&self) -> bool {
        self.status_callback_event
            .as_ref()
            .map(|e| e == &ConferenceEvent::ConferenceEnd)
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ConferenceEvent {
    ConferenceEnd,
    ConferenceStart,
    ParticipantLeave,
    ParticipantJoin,
    ParticipantMute,
    ParticipantUnmute,
    ParticipantHold,
    ParticipantUnhold,
    ParticipantModify,
    ParticipantSpeechStart,
    ParticipantSpeechStop,
    AnnouncementEnd,
    AnnouncementFail,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AMDRequestParams {
    pub call_sid: String,
    pub account_sid: String,
    pub answered_by: AnsweredBy,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AnsweredBy {
    MachineStart,
    Human,
    Fax,
    Unknown,
    MachineEndBeep,
    MachineEndSilence,
    MachineEndOther,
}
