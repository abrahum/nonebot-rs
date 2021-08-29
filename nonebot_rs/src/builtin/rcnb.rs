use crate::builtin;
use crate::event::MessageEvent;
use crate::matcher::{Handler, Matcher};
use crate::on_command;
use async_trait::async_trait;
use rcnb_rs::encode;

#[derive(Clone)]
pub struct Rcnb {}

#[async_trait]
impl Handler<MessageEvent> for Rcnb {
    on_command!(MessageEvent, "rcnb", "RCNB", "Rcnb");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let msg = event.get_raw_message();
        if !msg.is_empty() {
            let msg = encode(&msg);
            matcher.send_text(&msg).await;
        } else {
            let msg = matcher.request_message("Please enter something.").await;
            matcher.send_text(&encode(&msg)).await;
        }
    }
}

pub fn rcnb() -> Matcher<MessageEvent> {
    use std::sync::Arc;
    Matcher::new("Rcnb".to_string(), Rcnb {})
        .add_pre_matcher(builtin::prematcher::to_me())
        .add_pre_matcher(Arc::new(builtin::prematcher::command_start))
}
