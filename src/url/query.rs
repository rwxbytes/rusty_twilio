use crate::endpoints::accounts::Status;
use crate::endpoints::voice::call::CallStatus;
use crate::endpoints::QueryValues;

#[derive(Clone, Debug, Default)]
pub struct TwilioQuery<T> {
    pub params: QueryValues,
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

pub trait ByFriendlyName {}

impl<T: ByFriendlyName> TwilioQuery<T> {
    pub fn with_friendly_name(mut self, friendly_name: impl Into<String>) -> Self {
        self.params.push(("FriendlyName", friendly_name.into()));
        self
    }
}


pub trait AccountQueryMarker {}
impl<T: AccountQueryMarker> TwilioQuery<T> {
    pub fn with_status(mut self, status: Status) -> Self {
        self.params.push(("Status", status.to_string()));
        self
    }
}

pub trait ByToAndFrom {}

impl<T: ByToAndFrom> TwilioQuery<T> {
    pub fn with_to(mut self, to: impl Into<String>) -> Self {
        self.params.push(("To", to.into()));
        self
    }

    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.params.push(("From", from.into()));
        self
    }
}

pub trait CallQueryMarker {}

impl<T: CallQueryMarker> TwilioQuery<T> {
    pub fn with_parent_call_sid(mut self, parent_call_sid: impl Into<String>) -> Self {
        self.params.push(("ParentCallSid", parent_call_sid.into()));
        self
    }

    pub fn with_call_status(mut self, call_status: CallStatus) -> Self {
        self.params.push(("Status", call_status.to_string()));
        self
    }

    /// Only include calls that started on this date.
    /// Specify a date as YYYY-MM-DD in UTC, for example: 2009-07-06, to read only calls that started on this date.
    /// You can also specify an inequality, such as StartTime<=YYYY-MM-DD,
    /// to read calls that started on or before midnight of this date,
    /// and StartTime>=YYYY-MM-DD to read calls that started on or after midnight of this date.
    pub fn with_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.params.push(("StartTime", start_time.into()));
        self
    }

    /// Only include calls that ended on this date.
    /// Specify a date as YYYY-MM-DD in UTC, for example: 2009-07-06, to read only calls that ended on this date.
    /// You can also specify an inequality, such as EndTime<=YYYY-MM-DD,
    /// to read calls that ended on or before midnight of this date,
    /// and EndTime>=YYYY-MM-DD to read calls that ended on or after midnight of this date.
    pub fn with_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.params.push(("EndTime", end_time.into()));
        self
    }
}

pub trait ByDateCreatedAndDateUpdated {}

impl<T: ByDateCreatedAndDateUpdated> TwilioQuery<T> {
    pub fn with_date_created(mut self, date_created: impl Into<String>) -> Self {
        self.params.push(("DateCreated", date_created.into()));
        self
    }

    pub fn with_date_updated(mut self, date_updated: impl Into<String>) -> Self {
        self.params.push(("DateUpdated", date_updated.into()));
        self
    }
}

pub trait ConferenceQueryMarker {}

impl<T: ConferenceQueryMarker> TwilioQuery<T> {
    pub fn with_conference_status(mut self, status: impl Into<String>) -> Self {
        self.params.push(("Status", status.into()));
        self
    }
}