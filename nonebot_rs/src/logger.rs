use crate::event::{Event, MessageEvent, MetaEvent};
use async_trait::async_trait;
use colored::*;
use tracing::{event, Level};

/// Message Event Logger
pub fn message_logger(event: &MessageEvent) {
    match &event {
        MessageEvent::Private(p) => {
            let mut user_id = p.user_id.to_string();
            while user_id.len() < 10 {
                user_id.insert(0, ' ');
            }
            event!(
                Level::INFO,
                "{} [{}] -> {} from {}({})",
                user_id.green(),
                p.self_id.to_string().red(),
                p.raw_message,
                p.sender.nickname.to_string().blue(),
                p.user_id.to_string().green(),
            )
        }
        MessageEvent::Group(g) => {
            let mut group_id = g.group_id.to_string();
            while group_id.len() < 10 {
                group_id.insert(0, ' ');
            }
            event!(
                Level::INFO,
                "{} [{}] -> {} from {}({})",
                group_id.magenta(),
                g.self_id.to_string().red(),
                g.raw_message,
                g.sender.nickname.to_string().blue(),
                g.user_id.to_string().green(),
            )
        }
    }
}

/// Meta Event Logger
pub fn meta_logger(event: &MetaEvent) {
    if &event.meta_event_type == "heartbeat" {
        event!(Level::TRACE, "Recive HeartBeat")
    }
}

#[derive(Debug, Clone)]
pub struct Logger;

impl Logger {
    async fn event_recv(self, mut event_receiver: crate::EventReceiver) {
        while let Ok(event) = event_receiver.recv().await {
            match &event {
                Event::Message(m) => message_logger(m),
                Event::Meta(m) => meta_logger(m),
                _ => {}
            }
        }
    }
}

#[async_trait]
impl crate::Plugin for Logger {
    fn run(&self, event_receiver: crate::EventReceiver, _: crate::BotGetter) {
        let l = self.clone();
        tokio::spawn(l.event_recv(event_receiver));
    }

    fn plugin_name(&self) -> &'static str {
        "Logger"
    }

    async fn load_config(&mut self, _: toml::Value) {}
}
