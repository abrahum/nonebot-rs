use super::{build_temp_message_event_matcher, Handler, Matcher};
use crate::bot::ChannelItem;
use crate::event::{MessageEvent, SelfId};
use async_trait::async_trait;
use colored::*;
use tracing::{event as tevent, Level};

impl Matcher<MessageEvent> {
    pub async fn send_text(&self, msg: &str) {
        let msg = crate::message::Message::Text {
            text: msg.to_string(),
        };
        self.send(vec![msg]).await;
    }

    pub async fn set_temp_message_event_matcher<H>(&self, event: &MessageEvent, handler: H)
    where
        H: Handler<MessageEvent> + Send + Sync + 'static,
    {
        self.set_message_matcher(
            event.get_self_id(),
            build_temp_message_event_matcher(event, handler),
        )
        .await;
    }

    pub async fn request_message(
        &self,
        event: Option<&MessageEvent>,
        msg: Option<&str>,
    ) -> Option<String> {
        if let Some(event) = event {
            let raw_message = event.get_raw_message();
            if !raw_message.is_empty() {
                return Some(crate::utils::remove_space(raw_message));
            }
        }
        struct Temp {}

        #[async_trait]
        impl Handler<MessageEvent> for Temp {
            crate::on_match_all!();
            async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
                matcher
                    .sender
                    .clone()
                    .unwrap()
                    .send(crate::bot::ChannelItem::MessageEvent(event))
                    .await
                    .unwrap();
            }

            fn timeout_drop(&self, matcher: &Matcher<MessageEvent>) {
                let sender = matcher.sender.clone().unwrap();
                tokio::spawn(async move {
                    sender.send(crate::bot::ChannelItem::TimeOut).await.unwrap()
                });
            }
        }

        let (sender, mut receiver) = tokio::sync::mpsc::channel::<crate::bot::ChannelItem>(4);
        let event = self.event.clone().unwrap();
        self.set_message_matcher(
            event.get_self_id(),
            build_temp_message_event_matcher(&event, Temp {}).set_sender(sender),
        )
        .await;

        if let Some(msg) = msg {
            self.send_text(msg).await;
        }
        while let Some(data) = receiver.recv().await {
            match data {
                crate::bot::ChannelItem::MessageEvent(event) => {
                    let msg = crate::utils::remove_space(event.get_raw_message());
                    if msg.is_empty() {
                        return None;
                    } else {
                        return Some(msg);
                    }
                }
                crate::bot::ChannelItem::TimeOut => return None,
                // 中转 temp Matcher 的 Remove Action
                crate::bot::ChannelItem::Action(action) => self.set(action).await,
                _ => {
                    use colored::*;
                    tracing::event!(
                        tracing::Level::WARN,
                        "{}",
                        "Temp Matcher接受端接收到错误Api或Action消息".bright_red()
                    );
                } // 忽视 event 该 receiver 永不应该收到 event
            }
        }

        None
    }

    pub async fn send(&self, msg: Vec<crate::message::Message>) {
        match self.event.clone().unwrap() {
            MessageEvent::Private(p) => {
                let info = format!("Send {:?} to {}({})", msg, p.sender.nickname, p.user_id,);
                tevent!(
                    Level::INFO,
                    "Send {:?} to {}({})",
                    msg,
                    p.sender.nickname.blue(),
                    p.user_id.to_string().green(),
                );
                &self
                    .sender
                    .clone()
                    .unwrap()
                    .send(ChannelItem::Api(crate::api::Api::SendPrivateMsg {
                        params: crate::api::SendPrivateMsg {
                            user_id: p.user_id,
                            message: msg,
                            auto_escape: false,
                        },
                        echo: info,
                    }))
                    .await
                    .unwrap();
            }
            MessageEvent::Group(g) => {
                let info = format!("Send {:?} to group ({})", msg, g.group_id,);
                tevent!(
                    Level::INFO,
                    "Send {:?} to group ({})",
                    msg,
                    g.group_id.to_string().magenta(),
                );
                self.sender
                    .clone()
                    .unwrap()
                    .send(ChannelItem::Api(crate::api::Api::SendGroupMsg {
                        params: crate::api::SendGroupMsg {
                            group_id: g.group_id,
                            message: msg,
                            auto_escape: false,
                        },
                        echo: info,
                    }))
                    .await
                    .unwrap();
            }
        }
    }
}
