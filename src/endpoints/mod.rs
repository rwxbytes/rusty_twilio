pub mod accounts;
pub mod applications;

use std::collections::HashMap;
pub use crate::Result;

use reqwest::{Method, Response, Url};
pub use serde::{Deserialize, Serialize};
use crate::endpoints::accounts::Status;

#[derive(Debug)]
pub enum RequestBody {
    Empty,
    Json(serde_json::Value),
    Multipart(reqwest::multipart::Form),
    Form(HashMap<String, String>),
}

type QueryValues = Vec<(&'static str, String)>;


#[derive(Clone, Debug, Default)]
pub enum Region {
    Australia,
    Ireland,
    #[default]
    UnitedStates,
}

#[allow(async_fn_in_trait)]
pub trait TwilioEndpoint{
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

#[derive(Clone, Debug, Default)]
pub struct TwilioQuery<T> {
    params: QueryValues,
    _marker: std::marker::PhantomData<T>,
}

impl<T> TwilioQuery<T> {
    pub fn new() -> Self {
        Self {
            params: vec![],
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> TwilioQuery<T> {
    pub fn with_friendly_name(mut self, friendly_name: impl Into<String>) -> Self {
        self.params.push(("FriendlyName", friendly_name.into()));
        self
    }

    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.params.push(("PageSize", page_size.to_string()));
        self
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.params.push(("Page", page.to_string()));
        self
    }

    pub fn with_page_token(mut self, page_token: impl Into<String>) -> Self {
        self.params.push(("PageToken", page_token.into()));
        self
    }
}

trait AccountQueryMarker {}
impl<T: AccountQueryMarker> TwilioQuery<T> {
    pub fn with_status(mut self, status: Status) -> Self {
        self.params.push(("Status", status.to_string()));
        self
    }
}

