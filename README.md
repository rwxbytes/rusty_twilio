### Create Call

```rust
use rusty_twilio::endpoints::voice::call::{CreateCall, CreateCallBody};
use rusty_twilio::{Result, TwilioClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = TwilioClient::from_env()?;

    let body = CreateCallBody::new("to", "from", "http://demo.twilio.com/docs/voice.xml");

    // or to set other fields
    //let body = CreateCallBody {
    //    to: "to",
    //    from: "from",
    //    url: Some("http://demo.twilio.com/docs/voice.xml"),
    //    status_callback: Some("http://example.com/callback"),
    //    status_callback_event_initiated: Some(true),
    //    status_callback_event_answered: Some(true),
    //    ..Default::default()
    //};

    let endpoint = CreateCall::new(client.account_sid(), body);

    let resp = client.hit(endpoint).await?;

    println!("{:?}", resp);
    Ok(())
}
```