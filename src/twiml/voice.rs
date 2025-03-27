use super::ToTwiML;
use crate::error::TwilioError;
use http::header::CONTENT_TYPE;
use http::{header::HeaderValue, Response};
use serde::{Deserialize, Serialize};
use strum::Display;
use twiml_derive::ToTwiML;
use validator::Validate;
use xml::writer::{EventWriter, XmlEvent};

#[derive(Debug, Clone, Default)]
pub struct VoiceResponse {
    pub verbs: Vec<Verb>,
}

impl VoiceResponse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_verb(mut self, verb: Verb) -> Self {
        self.verbs.push(verb);
        self
    }

    pub fn connect(mut self, noun: impl Into<Noun>) -> Self {
        self.verbs.push(Verb::Connect(noun.into()));
        self
    }

    pub fn dial(mut self, dial: impl Into<Dial>) -> Self {
        self.verbs.push(Verb::Dial(dial.into()));
        self
    }

    pub fn reject(mut self) -> Self {
        self.verbs.push(Verb::Reject);
        self
    }

    pub fn to_http_response(&self) -> Result<Response<Vec<u8>>, TwilioError> {
        let body = self.to_bytes()?;
        let mut response = Response::new(body.into());
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
        Ok(response)
    }

    pub fn to_string(&self) -> Result<String, TwilioError> {
        let bytes = self.to_bytes()?;
        Ok(String::from_utf8(bytes)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, TwilioError> {
        let mut writer = EventWriter::new(Vec::new());
        writer.write(XmlEvent::start_element("Response"))?;
        for verb in &self.verbs {
            let _ = match &verb {
                Verb::Connect(noun) => match &noun {
                    Noun::Stream(stream) => {
                        stream.validate()?;
                    }
                    _ => Err(TwilioError::UnsupportedNoun)?,
                },
                Verb::Dial(dial) => match &dial.noun {
                    Noun::Conference(conference) => {
                        conference.validate()?;
                    }
                    Noun::Number(_) => {}
                    _ => Err(TwilioError::UnsupportedNoun)?,
                },
                Verb::Reject => {}
            };
            verb.write_xml(&mut writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(writer.into_inner())
    }
}

#[derive(Debug, Clone)]
pub enum Verb {
    /// See [Connect](https://www.twilio.com/docs/voice/twiml/connect)
    Connect(Noun),
    /// See [Dial](https://www.twilio.com/docs/voice/twiml/dial)
    Dial(Dial),
    /// See [Reject](https://www.twilio.com/docs/voice/twiml/reject)
    Reject,
}

impl ToTwiML for Verb {
    fn write_xml(&self, writer: &mut EventWriter<Vec<u8>>) -> Result<(), TwilioError> {
        match self {
            Verb::Connect(noun) => {
                writer.write(XmlEvent::start_element("Connect"))?;
                noun.write_xml(writer)?;
                writer.write(XmlEvent::end_element())?;
                Ok(())
            }
            Verb::Dial(dial) => dial.write_xml(writer),

            // TODO: add attributes to reject
            Verb::Reject => {
                writer.write(XmlEvent::start_element("Reject"))?;
                writer.write(XmlEvent::end_element())?;
                Ok(())
            }
        }
    }
}

// TODO: enable multiple numbers
#[derive(Debug, Clone, ToTwiML, Validate)]
pub struct Dial {
    #[xml(content)]
    pub noun: Noun,
    #[xml(attribute = "action")]
    pub action: Option<String>,
    #[xml(attribute = "answerOnBridge")]
    pub answer_on_bridge: Option<bool>,
    #[xml(attribute = "callerId")]
    pub caller_id: Option<String>,
    #[xml(attribute = "callReason")]
    pub call_reason: Option<String>,
    #[xml(attribute = "hangupOnStar")]
    pub hangup_on_star: Option<bool>,
    #[xml(attribute = "method")]
    pub method: Option<String>,
    #[xml(attribute = "record")]
    pub record: Option<String>,
    #[xml(attribute = "recordingStatusCallback")]
    pub recording_status_callback: Option<String>,
    #[xml(attribute = "recordingStatusCallbackMethod")]
    pub recording_status_callback_method: Option<String>,
    #[validate(custom(function = "validate_recording_status_callback_event"))]
    #[xml(attribute = "recordingStatusCallbackEvent")]
    pub recording_status_callback_event: Option<String>,
    #[xml(attribute = "recordingTrack")]
    pub recording_track: Option<String>,
    #[xml(attribute = "referUrl")]
    pub refer_url: Option<String>,
    #[xml(attribute = "referMethod")]
    pub refer_method: Option<String>,
    #[xml(attribute = "ringTone")]
    pub ring_tone: Option<String>,
    #[xml(attribute = "timeLimit")]
    pub time_limit: Option<u32>,
    #[xml(attribute = "timeout")]
    pub timeout: Option<u32>,
    #[xml(attribute = "trim")]
    pub trim: Option<String>,
    #[xml(attribute = "sequential")]
    pub sequential: Option<bool>,
}

impl Dial {
    pub fn new(noun: impl Into<Noun>) -> Self {
        Self {
            noun: noun.into(),
            action: None,
            answer_on_bridge: None,
            caller_id: None,
            call_reason: None,
            hangup_on_star: None,
            method: None,
            record: None,
            recording_status_callback: None,
            recording_status_callback_method: None,
            recording_status_callback_event: None,
            recording_track: None,
            refer_url: None,
            refer_method: None,
            ring_tone: None,
            time_limit: None,
            timeout: None,
            trim: None,
            sequential: None,
        }
    }
}

fn validate_recording_status_callback_event(event: &str) -> Result<(), validator::ValidationError> {
    let valid_events = ["in-progress", "completed", "absent"];
    if !valid_events.contains(&event) {
        let err = validator::ValidationError::new("invalid_recording_status_callback_event")
            .with_message(format!("Invalid recording status callback event: {}", event).into());
        return Err(err);
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum Noun {
    Conference(Conference),
    Number(Number),
    Stream(Stream),
}

impl ToTwiML for Noun {
    fn write_xml(&self, writer: &mut EventWriter<Vec<u8>>) -> Result<(), TwilioError> {
        match self {
            Noun::Stream(stream) => stream.write_xml(writer),
            Noun::Conference(conference) => conference.write_xml(writer),
            Noun::Number(number) => number.write_xml(writer),
        }
    }
}

#[derive(Debug, Clone, ToTwiML)]
pub struct Number {
    #[xml(content)]
    pub number: String,
    #[xml(attribute = "action")]
    pub action: Option<String>,
    #[xml(attribute = "method")]
    pub method: Option<String>,
}

impl Number {
    pub fn new(number: impl Into<String>) -> Self {
        Self {
            number: number.into(),
            action: None,
            method: None,
        }
    }
}

impl From<Number> for Noun {
    fn from(number: Number) -> Self {
        Noun::Number(number)
    }
}

impl From<Number> for Dial {
    fn from(number: Number) -> Self {
        Dial::new(number)
    }
}

#[derive(Clone, Debug, ToTwiML, Validate)]
pub struct Conference {
    #[xml(content)]
    pub name: String,

    #[xml(attribute = "muted")]
    /// The muted attribute lets you specify whether a participant can speak on the conference.
    /// If this attribute is set to true, the participant will only be able to listen to people on the conference.
    /// This attribute defaults to false.
    pub muted: Option<bool>,

    #[xml(attribute = "beep")]
    /// The beep attribute lets you specify whether a notification beep is played to the conference
    /// when a participant joins or leaves the conference. Defaults to true.
    ///
    /// - true: Plays a beep both when a participant joins and when a participant leaves.
    ///
    /// - false:	Disables beeps for when participants both join and exit.
    ///
    /// - onEnter:	Only plays a beep when a participant joins. The beep will not be played when the participant exits.
    ///
    /// - onExit:	Will not play a beep when a participant joins; only plays a beep when the participant exits.
    pub beep: Option<String>,

    #[xml(attribute = "startConferenceOnEnter")]
    pub start_conference_on_enter: Option<bool>,

    #[xml(attribute = "endConferenceOnExit")]
    pub end_conference_on_exit: Option<bool>,

    #[validate(length(max = 128))]
    #[xml(attribute = "participantLabel")]
    /// A unique label for the participant which will be added into the conference as a result of executing the TwiML.
    /// The label provided here can be used subsequently to read or update participant attributes using the Twilio REST API.
    /// The participantLabel must be unique across all participants in the conference, and there is a max limit of 128 characters.
    pub participant_label: Option<String>,

    #[xml(attribute = "jitterBufferSize")]
    pub jitter_buffer_size: Option<String>,

    #[xml(attribute = "waitUrl")]
    pub wait_url: Option<String>,

    #[xml(attribute = "waitMethod")]
    pub wait_method: Option<String>,

    #[validate(range(max = 250))]
    #[xml(attribute = "maxParticipants")]
    /// This attribute indicates the maximum number of participants you want to allow within a named conference room.
    /// The maximum number of participants is 250.
    pub max_participants: Option<u32>,

    #[xml(attribute = "record")]
    pub record: Option<String>,

    #[xml(attribute = "region")]
    pub region: Option<String>,

    #[xml(attribute = "trim")]
    pub trim: Option<String>,

    #[xml(attribute = "coach")]
    pub coach: Option<String>,

    #[xml(attribute = "statusCallback")]
    pub status_callback: Option<String>,

    #[validate(custom(function = "validate_status_callback_event"))]
    #[xml(attribute = "statusCallbackEvent")]
    /// The statusCallbackEvent attribute allows you to specify which conference state changes should generate a Webhook
    /// to the URL specified in the statusCallback attribute.
    /// The available values are **start, end, join, leave, mute, hold, modify, speaker, and announcement**.
    /// To specify multiple values separate them with a space.
    pub status_callback_event: Option<String>,

    #[xml(attribute = "statusCallbackMethod")]
    pub status_callback_method: Option<String>,

    #[xml(attribute = "recordingStatusCallback")]
    pub recording_status_callback: Option<String>,

    #[xml(attribute = "recordingStatusCallbackMethod")]
    pub recording_status_callback_method: Option<String>,

    #[validate(custom(function = "validate_recording_status_callback_event"))]
    #[xml(attribute = "recordingStatusCallbackEvent")]
    pub recording_status_callback_event: Option<String>,
}

fn validate_status_callback_event(event: &str) -> Result<(), validator::ValidationError> {
    let valid_events = [
        "start",
        "end",
        "join",
        "leave",
        "mute",
        "hold",
        "modify",
        "speaker",
        "announcement",
    ];

    let events: Vec<&str> = event.split_whitespace().collect();
    for e in events {
        if !valid_events.contains(&e) {
            let err = validator::ValidationError::new("invalid_status_callback_event")
                .with_message(format!("Invalid status callback event: {}", e).into());
            return Err(err);
        }
    }
    Ok(())
}

impl Conference {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            muted: None,
            beep: None,
            start_conference_on_enter: None,
            end_conference_on_exit: None,
            participant_label: None,
            jitter_buffer_size: None,
            wait_url: None,
            wait_method: None,
            max_participants: None,
            record: None,
            region: None,
            trim: None,
            coach: None,
            status_callback: None,
            status_callback_event: None,
            status_callback_method: None,
            recording_status_callback: None,
            recording_status_callback_method: None,
            recording_status_callback_event: None,
        }
    }
}

impl From<Conference> for Noun {
    fn from(conference: Conference) -> Self {
        Noun::Conference(conference)
    }
}

impl From<Conference> for Dial {
    fn from(conference: Conference) -> Self {
        Dial::new(conference)
    }
}

#[derive(Debug, Clone, Validate, ToTwiML)]
pub struct Stream {
    #[validate(url, custom(function = "validate_wss_url"))]
    #[xml(attribute = "url")]
    pub url: String,
    #[xml(attribute = "name")]
    pub name: Option<String>,
    #[xml(attribute = "track")]
    pub track: Option<Track>,
    #[validate(url)]
    #[xml(attribute = "statusCallback")]
    pub status_callback: Option<String>,
    #[xml(attribute = "statusCallbackMethod")]
    pub status_callback_method: Option<String>,
    #[xml(content)]
    pub parameters: Option<Vec<Parameter>>,
}

impl Stream {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            name: None,
            track: None,
            status_callback: None,
            status_callback_method: None,
            parameters: None,
        }
    }
}

impl From<Stream> for Noun {
    fn from(stream: Stream) -> Self {
        Noun::Stream(stream)
    }
}

fn validate_wss_url(url: &str) -> Result<(), validator::ValidationError> {
    if !url.starts_with("wss://") {
        let err = validator::ValidationError::new("invalid_websocket_url")
            .with_message("URL must start with 'wss://'".into());
        return Err(err);
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Display, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Track {
    InboundTrack,
    OutboundTrack,
    BothTracks,
}

#[derive(Clone, Debug, ToTwiML)]
pub struct Parameter {
    #[xml(attribute = "name")]
    pub name: String,
    #[xml(attribute = "value")]
    pub value: String,
}

impl Parameter {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn voice_response_is_erring_when_verb_contains_invalid_noun() {
        let got = VoiceResponse::new()
            .connect(Conference::new("test"))
            .to_string();
        assert!(got.is_err());
        if let Err(e) = got {
            assert_eq!(e.to_string(), "unsupported noun");
        }
    }

    #[test]
    fn connect_stream_twiml_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://test.com/connect" /></Connect></Response>"#;
        let got = VoiceResponse::new()
            .connect(Stream::new("wss://test.com/connect"))
            .to_string()
            .unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn connect_stream_twiml_is_erring_when_url_is_not_wss() {
        let got = VoiceResponse::new()
            .connect(Stream::new("https://test.com/connect"))
            .to_string();

        assert!(got.is_err());
        if let Err(e) = got {
            assert_eq!(
                e.to_string(),
                "validation error: url: URL must start with 'wss://'"
            );
        }
    }

    #[test]
    fn test_all_stream_attr() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://test.com/connect" name="test" track="inbound_track" statusCallback="https://test.com/callback" statusCallbackMethod="POST" /></Connect></Response>"#;
        let stream = Stream {
            url: "wss://test.com/connect".to_string(),
            name: Some("test".to_string()),
            track: Some(Track::InboundTrack),
            status_callback: Some("https://test.com/callback".to_string()),
            status_callback_method: Some("POST".to_string()),
            parameters: None,
        };
        stream.validate().expect("Stream validation failed");

        let got = VoiceResponse::new().connect(stream).to_string().unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn stream_noun_is_nesting_parameter_nouns() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://mystream.ngrok.io/example"><Parameter name="FirstName" value="Jane" /><Parameter name="LastName" value="Doe" /></Stream></Connect></Response>"#;
        let stream = Stream {
            url: "wss://mystream.ngrok.io/example".to_string(),
            name: None,
            track: None,
            status_callback: None,
            status_callback_method: None,
            parameters: Some(vec![
                Parameter {
                    name: "FirstName".to_string(),
                    value: "Jane".to_string(),
                },
                Parameter {
                    name: "LastName".to_string(),
                    value: "Doe".to_string(),
                },
            ]),
        };
        stream.validate().expect("Stream validation failed");

        let got = VoiceResponse::new().connect(stream).to_string().unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn voice_response_is_turning_into_http_response() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://test.com/connect" /></Connect></Response>"#;
        let response = VoiceResponse::new()
            .connect(Stream::new("wss://test.com/connect"))
            .to_http_response()
            .unwrap();

        assert_eq!(response.body(), want.as_bytes());
    }

    #[test]
    fn reject_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Reject /></Response>"#;
        let got = VoiceResponse::new().reject().to_string().unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn basic_dial_number_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Dial><Number>415-123-4567</Number></Dial></Response>"#;
        let got = VoiceResponse::new()
            .dial(Number::new("415-123-4567"))
            .to_string()
            .unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_dial_with_attributes_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Dial action="/handleDialCallStatus" method="GET"><Number>415-123-4567</Number></Dial></Response>"#;
        let init_dial = Dial::new(Number::new("415-123-4567"));
        let updated_dial = Dial {
            noun: init_dial.noun,
            action: Some("/handleDialCallStatus".to_string()),
            method: Some("GET".to_string()),
            ..init_dial
        };

        updated_dial.validate().expect("dial validation failed");
        let got = VoiceResponse::new().dial(updated_dial).to_string().unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn dial_conference_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Dial><Conference>Room 1234</Conference></Dial></Response>"#;
        let got = VoiceResponse::new()
            .dial(Conference::new("Room 1234"))
            .to_string()
            .unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn dial_conference_with_attributes_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Dial><Conference startConferenceOnEnter="true" endConferenceOnExit="true">moderated-conference-room</Conference></Dial></Response>"#;

        let init_conf = Conference::new("moderated-conference-room");
        let updated_conf = Conference {
            name: init_conf.name,
            start_conference_on_enter: Some(true),
            end_conference_on_exit: Some(true),
            ..init_conf
        };

        let got = VoiceResponse::new().dial(updated_conf).to_string().unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn dial_conference_status_callback_events_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Dial><Conference statusCallback="https://myapp.com/events" statusCallbackEvent="start end join leave mute hold">EventedConf</Conference></Dial></Response>"#;

        let init_conf = Conference::new("EventedConf");
        let updated_conf = Conference {
            name: init_conf.name,
            status_callback: Some("https://myapp.com/events".to_string()),
            status_callback_event: Some("start end join leave mute hold".to_string()),
            ..init_conf
        };

        updated_conf
            .validate()
            .expect("conference validation failed");

        let got = VoiceResponse::new().dial(updated_conf).to_string().unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn dial_conference_is_erring_when_status_callback_event_is_invalid() {
        let init_conf = Conference::new("EventedConf");
        let updated_conf = Conference {
            name: init_conf.name,
            status_callback: Some("https://myapp.com/events".to_string()),
            status_callback_event: Some("start end join leave mute hold invalid_event".to_string()),
            ..init_conf
        };

        let got = VoiceResponse::new().dial(updated_conf).to_string();
        assert!(got.is_err());
        if let Err(e) = got {
            assert_eq!(
                e.to_string(),
                "validation error: status_callback_event: Invalid status callback event: invalid_event"
            );
        }
    }
}
