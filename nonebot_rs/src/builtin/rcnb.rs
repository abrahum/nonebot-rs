use crate::builtin;
use crate::event::{MessageEvent, SelfId};
use crate::matcher::build_temp_message_event_matcher;
use crate::matcher::{Handler, Matcher};
use crate::{on_command, on_match_all};
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
            // matcher_request! {{
            //     let msg = event.get_raw_message();
            //     matcher.send_text(&encode(&msg)).await;
            // }}
            // #[derive(Clone)]
            struct Temp {}

            #[async_trait]
            impl Handler<MessageEvent> for Temp {
                on_match_all!();
                async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
                    let msg = event.get_raw_message();
                    matcher.send_text(&encode(&msg)).await;
                }
            }

            matcher
                .set_message_matcher(
                    event.get_self_id(),
                    build_temp_message_event_matcher(&event, Temp {}),
                )
                .await;
            matcher.send_text("Please enter something.").await;
        }
    }
}

pub fn rcnb() -> Matcher<MessageEvent> {
    Matcher::new("Rcnb".to_string(), Rcnb {})
        .add_pre_matcher(builtin::prematcher::to_me())
        .add_pre_matcher(builtin::prematcher::command_start())
}
