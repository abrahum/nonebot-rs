use crate::builtin;
use crate::event::MessageEvent;
use crate::matcher::{build_temp_message_event_matcher, Handler, Matcher};
use crate::message::{Message, TextMessage};
use crate::{on_command, on_match_all};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct Echo {}

#[async_trait]
impl Handler<MessageEvent> for Echo {
    on_command!(MessageEvent, "echo", "Echo");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let msg = Message::Text(TextMessage {
            text: event.get_raw_message().to_string(),
        });
        matcher.send(vec![msg]).await;
    }
}

pub fn echo() -> Matcher<MessageEvent> {
    Matcher::new("Echo".to_string(), Arc::new(Echo {}))
        .add_pre_matcher(builtin::prematcher::to_me())
        .add_pre_matcher(Arc::new(builtin::prematcher::command_start))
}

#[derive(Clone)]
pub struct Echo2 {}

#[async_trait]
impl Handler<MessageEvent> for Echo2 {
    on_command!(MessageEvent, "echo mode", "Echo Mode");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        // echo whatever you say until exit
        matcher
            .send_text("Enter Echo Mode\nType :q! to exit.")
            .await;

        pub struct Echo2 {}
        #[async_trait]
        impl Handler<MessageEvent> for Echo2 {
            on_match_all!();
            async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
                if event.get_raw_message() != ":q!" {
                    matcher
                        .set_message_matcher(
                            event.get_self_id(),
                            build_temp_message_event_matcher(&event, Echo2 {})
                                .set_priority(0)
                                .set_temp(true),
                        )
                        .await;
                    matcher.send_text(event.get_raw_message()).await;
                } else {
                    matcher.send_text("Quit echo mode").await;
                }
            }
        }

        matcher
            .set_message_matcher(
                event.get_self_id(),
                build_temp_message_event_matcher(&event, Echo2 {}),
            )
            .await;
    }
}

pub fn echo2() -> Matcher<MessageEvent> {
    Matcher::new("Echo2".to_string(), Arc::new(Echo2 {}))
        .add_pre_matcher(builtin::prematcher::to_me())
        .add_pre_matcher(Arc::new(builtin::prematcher::command_start))
}
