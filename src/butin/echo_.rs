use crate::api::{Apis, SendPrivateMsg};
use crate::bot::ApiSender;
use crate::event::MessageEvent;
use crate::matcher::{AMNb, Handler, Matcher};
use crate::message::{Message, TextMessage};
use crate::results::HandlerResult;
use async_trait::async_trait;
use colored::*;
use tracing::{event, Level};

pub struct Echo {}

#[async_trait]
impl Handler<MessageEvent> for Echo {
    async fn handle(self, event: MessageEvent, amnb: AMNb, sender: ApiSender) -> HandlerResult {
        match &event {
            MessageEvent::Private(p) => {
                if p.raw_message.starts_with(r"\echo ") {
                    let msg_text = p.raw_message.replace(r"\echo ", "");
                    let msg = Message::Text(TextMessage {
                        text: msg_text.clone(),
                    });
                    event!(
                        Level::INFO,
                        "echo {} to {}({})",
                        msg_text,
                        p.sender.nickname.to_string().blue(),
                        p.user_id.to_string().green(),
                    );
                    sender
                        .send(Apis::SendPrivateMsg {
                            params: SendPrivateMsg {
                                user_id: p.user_id,
                                message: vec![msg],
                                auto_escape: false,
                            },
                            echo: "echo".to_string(),
                        })
                        .await
                        .unwrap();
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            MessageEvent::Group(_) => Ok(false),
        }
    }
}

pub fn builder() -> Matcher<Echo> {
    Matcher {
        rules: vec![],
        block: false,
        temp: false,
        handler: Echo {},
        disable: false,
        ignore_command_start: true,
    }
}
