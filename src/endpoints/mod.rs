pub mod accounts;
pub mod applications;
pub mod voice;

pub use crate::Result;

use reqwest::{Method, RequestBuilder, Response, Url};
pub use serde::{Deserialize, Serialize};

#[allow(async_fn_in_trait)]
pub trait TwilioEndpoint {
    const PATH: &'static str;

    const METHOD: Method;

    type ResponseBody;

    fn query_params(&self) -> Option<QueryValues> {
        None
    }

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![]
    }

    fn configure_request_body(self, builder: RequestBuilder) -> Result<RequestBuilder>
    where
        Self: Sized,
    {
        Ok(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody>;

    fn url(&self, base_url: &Url) -> Url {
        let mut url = base_url.clone();
        let mut path = Self::PATH.to_string();

        for (placeholder, id) in self.path_params() {
            path = path.replace(placeholder, id);
        }

        url.set_path(&path);

        if let Some(query_params) = self.query_params() {
            url.query_pairs_mut().extend_pairs(query_params);
        }
        url
    }
}

#[derive(Debug)]
pub enum RequestBody<T> {
    Form(T),
    Json(T),
    Multipart(reqwest::multipart::Form),
}

impl<T: Serialize> RequestBody<T> {
    pub fn configure(self, builder: RequestBuilder) -> Result<RequestBuilder> {
        match self {
            RequestBody::Form(data) => Ok(builder.form(&data)),
            RequestBody::Json(data) => Ok(builder.json(&data)),
            RequestBody::Multipart(form) => Ok(builder.multipart(form)),
        }
    }
}

pub(crate) type QueryValues = Vec<(&'static str, String)>;

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
