use crate::builtin::prematcher::*;
use crate::event::MessageEvent;
use crate::matcher::{Handler, Matcher};
use crate::message::{Message, TextMessage};
use crate::on_command;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct Echo {}

#[async_trait]
impl Handler<MessageEvent> for Echo {
    on_command!(MessageEvent, "echo");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let msg = Message::Text(TextMessage {
            text: event.get_raw_message().to_string(),
        });
        matcher.send(vec![msg]).await;
    }
}

pub fn echo() -> Matcher<MessageEvent> {
    Matcher::new("Echo".to_string(), Arc::new(Echo {}))
        .add_pre_matcher(to_me())
        .add_pre_matcher(Arc::new(command_start))
}
