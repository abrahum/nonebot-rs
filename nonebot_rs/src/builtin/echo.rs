use crate::builtin;
use crate::event::MessageEvent;
#[cfg(feature = "matcher")]
use crate::matcher::{Handler, Matcher};
use crate::message::Message;
use crate::on_command;
use async_trait::async_trait;

#[doc(hidden)]
#[derive(Clone)]
pub struct Echo {}

#[doc(hidden)]
#[async_trait]
impl Handler<MessageEvent> for Echo {
    on_command!(MessageEvent, "echo", "Echo");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let msg = Message::Text {
            text: event.get_raw_message().to_string(),
        };
        matcher.send(vec![msg]).await;
    }
}

/// 单次复读 Matcher
pub fn echo() -> Matcher<MessageEvent> {
    Matcher::new("Echo", Echo {})
        .add_pre_matcher(builtin::prematchers::to_me())
        .add_pre_matcher(builtin::prematchers::command_start())
}

#[doc(hidden)]
#[derive(Clone)]
pub struct Echo2 {}

#[doc(hidden)]
#[async_trait]
impl Handler<MessageEvent> for Echo2 {
    on_command!(MessageEvent, "echo mode", "Echo Mode");
    async fn handle(&self, _: MessageEvent, matcher: Matcher<MessageEvent>) {
        // echo whatever you say until exit
        matcher
            .send_text("Enter Echo Mode\nType :q! to exit.")
            .await;

        while let Some(msg) = matcher.request_message(None, None).await {
            if msg == ":q!" {
                matcher.send_text("Quit echo mode").await;
                break;
            } else {
                matcher.send_text(&msg).await;
            }
        }
    }
}

/// 无限复读 Matcher
pub fn echo2() -> Matcher<MessageEvent> {
    Matcher::new("Echo2", Echo2 {})
        .add_pre_matcher(builtin::prematchers::to_me())
        .add_pre_matcher(builtin::prematchers::command_start())
}
