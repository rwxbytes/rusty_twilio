//! Accounts endpoints
//! See [Twilio Accounts API](https://www.twilio.com/docs/iam/api/account)
use super::*;
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

#[derive(Clone, Debug, Deserialize, Display)]
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

#[derive(Clone, Debug)]
pub struct CreateAccount {
    pub friendly_name: String,
}

impl CreateAccount {
    pub fn new(friendly_name: impl Into<String>) -> Self {
        Self {
            friendly_name: friendly_name.into(),
        }
    }
}

impl TwilioEndpoint for CreateAccount {
    const PATH: &'static str = "2010-04-01/Accounts.json";
    const METHOD: Method = Method::POST;
    type ResponseBody = AccountResponse;

    fn request_body(&self) -> Result<RequestBody> {
        Ok(RequestBody::Form(vec![(
            "FriendlyName",
            self.friendly_name.clone(),
        )]))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
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

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListAccounts {
    query: ListAccountsQuery,
}

impl ListAccounts {
    pub fn new(query: ListAccountsQuery) -> Self {
        Self { query }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListAccountsQuery {
    params: QueryValues,
}

impl ListAccountsQuery {
    pub fn with_friendly_name(mut self, friendly_name: impl Into<String>) -> Self {
        self.params.push(("FriendlyName", friendly_name.into()));
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.params.push(("Status", status.to_string()));
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

impl TwilioEndpoint for ListAccounts {
    const PATH: &'static str = "2010-04-01/Accounts.json";
    const METHOD: Method = Method::GET;
    type ResponseBody = ListAccountsResponse;

    fn query_params(&self) -> Option<QueryValues> {
        Some(self.query.params.clone())
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountResponse>,
    pub end: u32,
    pub next_page_uri: Option<String>,
    pub page: u32,
    pub page_size: u32,
    pub previous_page_uri: Option<String>,
    pub start: u32,
    pub uri: String,
}

#[derive(Clone, Debug)]
pub struct UpdateAccount {
    pub account_sid: String,
    pub body: UpdateAccountBody,
}

impl UpdateAccount {
    pub fn new(account_sid: impl Into<String>, body: UpdateAccountBody) -> Self {
        Self {
            account_sid: account_sid.into(),
            body,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UpdateAccountBody {
    pub friendly_name: Option<String>,
    pub status: Option<Status>,
}

impl UpdateAccountBody {
    pub fn with_friendly_name(mut self, friendly_name: impl Into<String>) -> Self {
        self.friendly_name = Some(friendly_name.into());
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }
}

impl TwilioEndpoint for UpdateAccount {
    const PATH: &'static str = "2010-04-01/Accounts/{Sid}.json";
    const METHOD: Method = Method::POST;
    type ResponseBody = AccountResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![("{Sid}", &self.account_sid)]
    }

    fn request_body(&self) -> Result<RequestBody> {
        let mut form = vec![];

        if let Some(friendly_name) = &self.body.friendly_name {
            form.push(("FriendlyName", friendly_name.clone()));
        }

        if let Some(status) = &self.body.status {
            form.push(("Status", status.to_string()));
        }

        Ok(RequestBody::Form(form))
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}
