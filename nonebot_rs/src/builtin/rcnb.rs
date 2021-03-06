use crate::matcher::prelude::*;
use rcnb_rs::encode;

#[doc(hidden)]
#[derive(Clone)]
pub struct Rcnb {}

#[doc(hidden)]
#[async_trait]
impl Handler<MessageEvent> for Rcnb {
    on_command!(MessageEvent, "rcnb", "RCNB", "Rcnb");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let msg = matcher
            .request_message(Some(&event), Some("Please enter something."))
            .await;
        if let Some(msg) = msg {
            matcher.send_text(&encode(&msg)).await;
        }
    }
}

/// rcnb！！！
pub fn rcnb() -> Matcher<MessageEvent> {
    Matcher::new("Rcnb", Rcnb {})
        .add_pre_matcher(prematchers::to_me())
        .add_pre_matcher(prematchers::command_start())
}
