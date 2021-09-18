use crate::matcher::prelude::*;

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
        .add_pre_matcher(prematchers::to_me())
        .add_pre_matcher(prematchers::command_start())
}

#[doc(hidden)]
#[derive(Clone)]
pub struct Echo2 {
    max_times: i64, // negative for infinite (all most)
}

#[doc(hidden)]
#[async_trait]
impl Handler<MessageEvent> for Echo2 {
    on_command!(MessageEvent, "echo mode", "Echo Mode");
    async fn handle(&self, _: MessageEvent, matcher: Matcher<MessageEvent>) {
        // echo whatever you say until exit
        let mut max_times = self.max_times;
        matcher
            .send_text("Enter Echo Mode\nType :q! to exit.")
            .await;

        while let Some(msg) = matcher.request_message(None, None).await {
            if msg == ":q!" {
                matcher.send_text("Quit echo mode").await;
                break;
            } else {
                matcher.send_text(&msg).await;
                max_times -= 1;
            }

            if max_times == 0 {
                matcher.send_text("Quit echo mode").await;
                break;
            }
        }
    }

    fn load_config(&mut self, config: std::collections::HashMap<String, toml::Value>) {
        if let Some(data) = config.get("max_times") {
            self.max_times = data.clone().try_into().expect("max_times 不是正整数");
        }
        use tracing::{event, Level};
        event!(Level::DEBUG, "Load max echo times:{}", self.max_times);
    }
}

/// 无限复读 Matcher
pub fn echo2() -> Matcher<MessageEvent> {
    Matcher::new("Echo2", Echo2 { max_times: 0 })
        .add_pre_matcher(prematchers::to_me())
        .add_pre_matcher(prematchers::command_start())
}
