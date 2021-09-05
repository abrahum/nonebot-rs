use super::{build_temp_message_event_matcher, Handler, Matcher};
use crate::event::MessageEvent;
use crate::ApiChannelItem;
use async_trait::async_trait;
use colored::*;
use tracing::{event as tevent, Level};

impl Matcher<MessageEvent> {
    /// 发送纯文本消息
    pub async fn send_text(&self, msg: &str) {
        let msg = crate::message::Message::Text {
            text: msg.to_string(),
        };
        self.send(vec![msg]).await;
    }

    /// 设置临时 Matcher<MessageEvent>
    pub async fn set_temp_message_event_matcher<H>(&self, event: &MessageEvent, handler: H)
    where
        H: Handler<MessageEvent> + Send + Sync + 'static,
    {
        self.set_message_matcher(build_temp_message_event_matcher(event, handler))
            .await;
    }

    /// 请求消息内容
    ///
    /// 传入 event raw_message 若不为空则直接返回该消息文本（传入 None 表示必须请求）
    ///
    /// 传入 msg 为发送给用户的请求文本信息（传入 None 表示不向用户发送请求信息）
    ///
    /// 重新请求消息为空将返回 None
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
                    .api_sender
                    .clone()
                    .unwrap()
                    .send(ApiChannelItem::MessageEvent(event))
                    .await
                    .unwrap();
            }

            fn timeout_drop(&self, matcher: &Matcher<MessageEvent>) {
                let sender = matcher.api_sender.clone().unwrap();
                tokio::spawn(async move { sender.send(ApiChannelItem::TimeOut).await.unwrap() });
            }
        }

        let (sender, mut receiver) = tokio::sync::mpsc::channel::<ApiChannelItem>(4);
        let event = self.event.clone().unwrap();
        let mut m = build_temp_message_event_matcher(&event, Temp {});
        m.api_sender = Some(sender);
        self.set_message_matcher(m).await;

        if let Some(msg) = msg {
            self.send_text(msg).await;
        }
        while let Some(data) = receiver.recv().await {
            match data {
                ApiChannelItem::MessageEvent(event) => {
                    let msg = crate::utils::remove_space(event.get_raw_message());
                    if msg.is_empty() {
                        return None;
                    } else {
                        return Some(msg);
                    }
                }
                ApiChannelItem::TimeOut => return None,
                // 中转 temp Matcher 的 Remove Action
                ApiChannelItem::Action(action) => self.set(action).await,
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

    /// 发送 Vec<Message> 消息
    pub async fn send(&self, msg: Vec<crate::message::Message>) {
        match self.event.clone().unwrap() {
            MessageEvent::Private(p) => {
                tevent!(
                    Level::INFO,
                    "Bot [{}] Send {:?} to {}({})",
                    p.self_id.to_string().red(),
                    msg,
                    p.sender.nickname.blue(),
                    p.user_id.to_string().green(),
                );
                &self
                    .api_sender
                    .clone()
                    .unwrap()
                    .send(ApiChannelItem::Api(crate::Api::send_private_msg(
                        crate::SendPrivateMsg {
                            user_id: p.user_id,
                            message: msg,
                            auto_escape: false,
                        },
                    )))
                    .await
                    .unwrap();
            }
            MessageEvent::Group(g) => {
                tevent!(
                    Level::INFO,
                    "Bot [{}] Send {:?} to group ({})",
                    g.self_id.to_string().red(),
                    msg,
                    g.group_id.to_string().magenta(),
                );
                self.api_sender
                    .clone()
                    .unwrap()
                    .send(ApiChannelItem::Api(crate::Api::send_group_msg(
                        crate::SendGroupMsg {
                            group_id: g.group_id,
                            message: msg,
                            auto_escape: false,
                        },
                    )))
                    .await
                    .unwrap();
            }
        }
    }
}
