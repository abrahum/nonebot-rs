use crate::config::BotConfig;
use crate::event::MessageEvent;
use crate::matcher::PreMatcher;
use crate::message::Message;
use crate::utils::remove_space;
use std::sync::Arc;

pub fn to_me() -> Arc<PreMatcher<MessageEvent>> {
    let to_me = |e: &mut MessageEvent, config: BotConfig| -> bool {
        match e {
            MessageEvent::Private(_) => true,
            MessageEvent::Group(g) => {
                let bot_id = g.self_id.to_string();
                let raw_message = remove_space(&g.raw_message);
                for name in config.nickname {
                    if raw_message.starts_with(&name) {
                        g.raw_message = remove_space(&raw_message[name.len()..]);
                        return true;
                    }
                }
                for message in &g.message {
                    match message {
                        Message::At(at) => {
                            if at.qq == bot_id {
                                return true;
                            }
                        }
                        _ => continue,
                    }
                }
                false
            }
        }
    };

    Arc::new(to_me)
}

pub fn command_start(event: &mut MessageEvent, config: BotConfig) -> bool {
    let raw_message = remove_space(&event.get_raw_message());
    let command_starts = config.command_start;
    if command_starts.is_empty() {
        return true;
    }
    for sc in &command_starts {
        if raw_message.starts_with(sc) {
            let new_raw_message = remove_space(&raw_message[sc.len()..]);
            event.set_raw_message(new_raw_message);
            return true;
        }
    }
    false
}
