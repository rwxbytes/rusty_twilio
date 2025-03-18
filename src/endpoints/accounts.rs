//! Accounts endpoints
//! See [Twilio Accounts API](https://www.twilio.com/docs/iam/api/account)
#![allow(unused_imports)]
use super::*;
use crate::url::query::{AccountQueryMarker, ByFriendlyName, TwilioQuery};
use reqwest::RequestBuilder;
use std::string::ToString;
use strum::Display;

#[derive(Clone, Debug, Deserialize)]
/// See [Account Properties](https://www.twilio.com/docs/iam/api/account#account-properties)
pub struct AccountResponse {
    /// The authorization token for this account. This token should be kept a secret, so no sharing.
    pub auth_token: String,
    /// The date that this account was created, in GMT in RFC 2822 format
    pub date_created: String,
    /// The date that this account was last updated, in GMT in RFC 2822 format.
    pub date_updated: String,
    /// A human-readable description of this account, up to 64 characters long. By default, the FriendlyName is your email address.
    pub friendly_name: String,
    /// The unique 34 character id that represents the parent of this account. The OwnerAccountSid of a parent account is its own sid.
    pub owner_account_sid: String,
    /// A 34 character string that uniquely identifies this resource.
    pub sid: String,
    /// The status of this account. Usually active, but can be suspended or closed.
    pub status: Status,
    /// The type of this account. Either Trial or Full if it's been upgraded
    pub r#type: AccountType,
    /// The URI for this resource, relative to https://api.twilio.com
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Display, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Status {
    Active,
    Suspended,
    Closed,
}

#[derive(Clone, Debug, Deserialize, Display)]
#[strum(serialize_all = "lowercase")]
pub enum AccountType {
    Trial,
    Full,
}

#[derive(Debug)]
pub struct CreateAccount<'a> {
    pub body: RequestBody<CreateAccountBody<'a>>,
}

impl<'a> CreateAccount<'a> {
    pub fn new(friendly_name: &'a str) -> Self {
        Self {
            body: RequestBody::Form(CreateAccountBody {
                friendly_name: Some(friendly_name),
            }),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateAccountBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<&'a str>,
}

impl TwilioEndpoint for CreateAccount<'_> {
    const PATH: &'static str = "2010-04-01/Accounts.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = AccountResponse;

    fn configure_request_body(self, builder: RequestBuilder) -> Result<RequestBuilder> {
        self.body.configure(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug)]
pub struct FetchAccount {
    pub account_sid: String,
}

impl FetchAccount {
    pub fn new(account_sid: impl Into<String>) -> Self {
        Self {
            account_sid: account_sid.into(),
        }
    }
}

impl TwilioEndpoint for FetchAccount {
    const PATH: &'static str = "2010-04-01/Accounts/{Sid}.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = AccountResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{Sid}", &self.account_sid)]
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

impl ByFriendlyName for ListAccounts {}

impl AccountQueryMarker for ListAccounts {}

#[derive(Clone, Debug)]
pub struct ListAccounts {
    query: TwilioQuery<Self>,
}

impl ListAccounts {
    pub fn new(query: TwilioQuery<Self>) -> Self {
        Self { query }
    }
}

impl TwilioEndpoint for ListAccounts {
    const PATH: &'static str = "2010-04-01/Accounts.json";

    const METHOD: Method = Method::GET;

    type ResponseBody = ListAccountsResponse;

    fn query_params(&self) -> Option<QueryValues> {
        Some(self.query.params.clone())
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountResponse>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug)]
pub struct UpdateAccount<'a> {
    pub account_sid: String,
    pub body: RequestBody<UpdateAccountBody<'a>>,
}

impl<'a> UpdateAccount<'a> {
    pub fn new(account_sid: impl Into<String>, body: UpdateAccountBody<'a>) -> Self {
        Self {
            account_sid: account_sid.into(),
            body: RequestBody::Form(body),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateAccountBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl TwilioEndpoint for UpdateAccount<'_> {
    const PATH: &'static str = "2010-04-01/Accounts/{Sid}.json";

    const METHOD: Method = Method::POST;

    type ResponseBody = AccountResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{Sid}", &self.account_sid)]
    }

    fn configure_request_body(self, builder: RequestBuilder) -> Result<RequestBuilder> {
        self.body.configure(builder)
    }

    async fn response_body(resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}
