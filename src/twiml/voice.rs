use crate::error::TwilioError;
use http::header::CONTENT_TYPE;
use http::{header::HeaderValue, Response};
use serde::{Deserialize, Serialize};
use std::io::Write;
use url::Url;
use xml::writer::{EventWriter, XmlEvent};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize, Default)]
#[serde(rename = "Response")]
pub struct VoiceResponse {
    pub verbs: Vec<Verb>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum Verb {
    Connect(Noun),
    Reject,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum Noun {
    Stream(Stream),
}

impl From<&str> for Noun {
    fn from(url: &str) -> Self {
        Noun::Stream(Stream {
            url: url.to_string(),
            name: None,
            track: None,
            status_callback: None,
            status_callback_method: None,
            parameters: None,
        })
    }
}

impl From<String> for Noun {
    fn from(url: String) -> Self {
        Noun::Stream(Stream {
            url,
            name: None,
            track: None,
            status_callback: None,
            status_callback_method: None,
            parameters: None,
        })
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Stream {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<Track>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Track {
    InboundTrack,
    OutboundTrack,
    BothTracks,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

impl From<Stream> for Noun {
    fn from(stream: Stream) -> Self {
        Noun::Stream(stream)
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamNounBuilder {
    url: Option<String>,
    name: Option<String>,
    track: Option<Track>,
    status_callback: Option<String>,
    status_callback_method: Option<String>,
    parameters: Option<Vec<Parameter>>,
}

impl StreamNounBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Result<Self, TwilioError> {
        let url_str = url.into();
        if let Err(_) = Url::parse(&url_str) {
            return Err(TwilioError::InvalidWebSocketUrl(url_str));
        }

        if !url_str.starts_with("wss://") {
            return Err(TwilioError::InvalidWebSocketUrl(
                "URL must start with 'wss://'".to_string(),
            ));
        }

        self.url = Some(url_str);
        Ok(self)
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn track(mut self, track: Track) -> Self {
        self.track = Some(track);
        self
    }

    pub fn status_callback(mut self, callback: impl Into<String>) -> Result<Self, TwilioError> {
        let callback_str = callback.into();
        if let Err(_) = Url::parse(&callback_str) {
            return Err(TwilioError::InvalidCallbackUrl(callback_str));
        }
        self.status_callback = Some(callback_str);
        Ok(self)
    }

    pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.status_callback_method = Some(method.into());
        self
    }

    pub fn parameter(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let parameter = Parameter {
            name: name.into(),
            value: value.into(),
        };

        if let Some(parameters) = &mut self.parameters {
            parameters.push(parameter);
        } else {
            self.parameters = Some(vec![parameter]);
        }

        self
    }

    pub fn build(self) -> Result<Stream, TwilioError> {
        let url = self.url.ok_or_else(|| {
            TwilioError::InvalidWebSocketUrl("WebSocket URL is required".to_string())
        })?;

        Ok(Stream {
            url,
            name: self.name,
            track: self.track,
            status_callback: self.status_callback,
            status_callback_method: self.status_callback_method,
            parameters: self.parameters,
        })
    }
}

impl VoiceResponse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, noun: impl Into<Noun>) -> &mut Self {
        self.verbs.push(Verb::Connect(noun.into()));
        self
    }

    pub fn reject(mut self) -> Self {
        self.verbs.push(Verb::Reject);
        self
    }

    pub fn to_http_response(&self) -> Result<Response<String>, TwilioError> {
        let body = self.to_string()?;
        let mut response = Response::new(body.into());
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
        Ok(response)
    }

    pub fn to_string(&self) -> Result<String, TwilioError> {
        let w = Vec::new();
        let mut writer = EventWriter::new(w);

        writer.write(XmlEvent::start_element("Response"))?;

        for verb in &self.verbs {
            match verb {
                Verb::Connect(noun) => {
                    writer.write(XmlEvent::start_element("Connect"))?;

                    match noun {
                        Noun::Stream(stream) => {
                            if let Err(_) = Url::parse(&stream.url) {
                                return Err(TwilioError::InvalidWebSocketUrl(stream.url.clone()));
                            }

                            if !stream.url.starts_with("wss://") {
                                return Err(TwilioError::InvalidWebSocketUrl(
                                    "URL must start with 'wss://'".to_string(),
                                ));
                            }

                            let mut stream_element =
                                XmlEvent::start_element("Stream").attr("url", &stream.url);

                            if let Some(name) = &stream.name {
                                stream_element = stream_element.attr("name", name);
                            }
                            if let Some(track) = &stream.track {
                                stream_element = stream_element.attr(
                                    "track",
                                    match track {
                                        Track::InboundTrack => "inbound_track",
                                        Track::OutboundTrack => "outbound_track",
                                        Track::BothTracks => "both_tracks",
                                    },
                                );
                            }
                            if let Some(callback) = &stream.status_callback {
                                stream_element = stream_element.attr("statusCallback", callback);
                            }
                            if let Some(method) = &stream.status_callback_method {
                                stream_element =
                                    stream_element.attr("statusCallbackMethod", method);
                            }

                            writer.write(stream_element)?;

                            if let Some(parameters) = &stream.parameters {
                                for parameter in parameters {
                                    let parameter_element = XmlEvent::start_element("Parameter")
                                        .attr("name", &parameter.name)
                                        .attr("value", &parameter.value);

                                    writer.write(parameter_element)?;
                                    writer.write(XmlEvent::end_element().name("Parameter"))?;
                                }
                            }

                            writer.write(XmlEvent::end_element().name("Stream"))?;
                        }
                    }

                    writer.write(XmlEvent::end_element().name("Connect"))?;
                }
                Verb::Reject => {
                    writer.write(XmlEvent::start_element("Reject"))?;
                    writer.write(XmlEvent::end_element().name("Reject"))?;
                }
            }
        }

        writer.write(XmlEvent::end_element().name("Response"))?;

        let buffer = writer.into_inner();
        Ok(String::from_utf8(buffer)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn connect_stream_twiml_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://test.com/connect" /></Connect></Response>"#;
        let got = VoiceResponse::new()
            .connect("wss://test.com/connect")
            .to_string()
            .unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn connect_stream_twiml_is_erring_when_url_is_not_wss() {
        let got = VoiceResponse::new()
            .connect("https://test.com/connect")
            .to_string();

        assert!(got.is_err());
    }

    #[test]
    fn test_all_stream_attr() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://test.com/connect" name="test" track="inbound_track" statusCallback="https://test.com/callback" statusCallbackMethod="POST" /></Connect></Response>"#;
        let stream = StreamNounBuilder::new()
            .url("wss://test.com/connect")
            .unwrap()
            .name("test")
            .track(Track::InboundTrack)
            .status_callback("https://test.com/callback")
            .unwrap()
            .status_callback_method("POST")
            .build()
            .unwrap();

        let got = VoiceResponse::new().connect(stream).to_string().unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn stream_noun_is_nesting_parameter_nouns() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://mystream.ngrok.io/example"><Parameter name="FirstName" value="Jane" /><Parameter name="LastName" value="Doe" /></Stream></Connect></Response>"#;

        let stream = StreamNounBuilder::new()
            .url("wss://mystream.ngrok.io/example")
            .unwrap()
            .parameter("FirstName", "Jane")
            .parameter("LastName", "Doe")
            .build()
            .unwrap();

        let got = VoiceResponse::new().connect(stream).to_string().unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn voice_response_is_turning_into_http_response() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Connect><Stream url="wss://test.com/connect" /></Connect></Response>"#;
        let response = VoiceResponse::new()
            .connect("wss://test.com/connect")
            .to_http_response()
            .unwrap();

        assert_eq!(response.body(), want);
    }

    #[test]
    fn reject_is_constructing() {
        let want = r#"<?xml version="1.0" encoding="UTF-8"?><Response><Reject /></Response>"#;
        let got = VoiceResponse::new().reject().to_string().unwrap();

        assert_eq!(got, want);
    }
}
