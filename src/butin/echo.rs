use crate::bot::ApiSender;
use crate::event::MessageEvent;
use crate::matcher::{Handler, Matcher};
use crate::message::{Message, TextMessage};
use crate::results::HandlerResult;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct Echo {}

#[async_trait]
impl Handler<MessageEvent> for Echo {
    async fn handle(&self, event: MessageEvent, sender: ApiSender) -> HandlerResult {
        if event.get_raw_message().starts_with(r"echo ") {
            let msg_text = event.get_raw_message().replace(r"echo ", "");
            let msg = Message::Text(TextMessage {
                text: msg_text.clone(),
            });
            event.send(sender, vec![msg]).await;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn echo() -> Matcher<MessageEvent> {
    Matcher {
        pre_matchers: vec![
            crate::butin::prematcher::to_me(), // 使用构造函数传递
            Arc::new(crate::butin::prematcher::command_start), // 使用函数名传递
        ],
        after_matchers: vec![],
        rules: vec![],
        block: false,
        handler: Arc::new(Echo {}),
        disable: false,
        ignore_command_start: true,
    }
}
