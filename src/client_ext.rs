#![allow(dead_code)]
use crate::endpoints::voice::call::{CreateCall, CreateCallBody, UpdateCall, UpdateCallBody};
use crate::endpoints::TwilioEndpoint;
use crate::{Result, TwilioClient};
use std::future::Future;

// Trait definition for common Twilio operations
pub trait TwilioClientExt {
    fn create_call_with_twiml(
        &self,
        to: &str,
        from: &str,
        url: &str,
    ) -> impl Future<Output = Result<<CreateCall as TwilioEndpoint>::ResponseBody>>;

    fn create_call_with_url(
        &self,
        to: &str,
        from: &str,
        url: &str,
    ) -> impl Future<Output = Result<<CreateCall as TwilioEndpoint>::ResponseBody>>;

    fn update_call_with_twiml(
        &self,
        call_sid: &str,
        twiml: &str,
    ) -> impl Future<Output = Result<<UpdateCall as TwilioEndpoint>::ResponseBody>>;

    fn update_call_with_url(
        &self,
        call_sid: &str,
        url: &str,
    ) -> impl Future<Output = Result<<UpdateCall as TwilioEndpoint>::ResponseBody>>;
}

impl TwilioClientExt for TwilioClient {
    async fn create_call_with_twiml(
        &self,
        to: &str,
        from: &str,
        twiml: &str,
    ) -> Result<<CreateCall as TwilioEndpoint>::ResponseBody> {
        let body = CreateCallBody {
            to,
            from,
            twiml: Some(twiml),
            ..Default::default()
        };
        let endpoint = CreateCall::new(self.account_sid(), body);
        self.hit(endpoint).await
    }

    async fn create_call_with_url(
        &self,
        to: &str,
        from: &str,
        url: &str,
    ) -> Result<<CreateCall as TwilioEndpoint>::ResponseBody> {
        let body = CreateCallBody::new(to, from, url);
        let endpoint = CreateCall::new(self.account_sid(), body);
        self.hit(endpoint).await
    }

    async fn update_call_with_twiml(
        &self,
        call_sid: &str,
        twiml: &str,
    ) -> Result<<UpdateCall as TwilioEndpoint>::ResponseBody> {
        let body = UpdateCallBody {
            twiml: Some(twiml),
            ..Default::default()
        };
        let endpoint = UpdateCall::new(self.account_sid(), call_sid, body);
        self.hit(endpoint).await
    }

    async fn update_call_with_url(
        &self,
        call_sid: &str,
        url: &str,
    ) -> Result<<UpdateCall as TwilioEndpoint>::ResponseBody> {
        let body = UpdateCallBody {
            url: Some(url),
            ..Default::default()
        };
        let endpoint = UpdateCall::new(self.account_sid(), call_sid, body);
        self.hit(endpoint).await
    }
}
