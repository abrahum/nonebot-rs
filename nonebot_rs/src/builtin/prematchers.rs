use crate::config::BotConfig;
use crate::event::MessageEvent;
use crate::matcher::PreMatcher;
use crate::message::Message;
use crate::utils::remove_space;
use std::sync::Arc;

/// 判定消息是否提及 bot（私聊，at，昵称）
pub fn to_me() -> Arc<PreMatcher<MessageEvent>> {
    let to_me = |e: &mut MessageEvent, config: BotConfig| -> bool {
        match e {
            MessageEvent::Private(_) => true,
            MessageEvent::Group(g) => {
                let bot_id = g.self_id.to_string();
                let raw_message = remove_space(&g.raw_message);
                for name in config.nicknames {
                    if raw_message.starts_with(&name) {
                        g.raw_message = remove_space(&raw_message[name.len()..]);
                        return true;
                    }
                }
                for message in &g.message {
                    match message {
                        Message::At { qq: qq_id } => {
                            if qq_id == &bot_id {
                                g.raw_message = remove_space(
                                    &raw_message.replace(&format!("[CQ:at,qq={}]", g.self_id), ""),
                                );
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

#[doc(hidden)]
fn command_start_(event: &mut MessageEvent, config: BotConfig) -> bool {
    let raw_message = remove_space(&event.get_raw_message());
    let command_starts = config.command_starts;
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

/// 判定消息是否符合命令起始符
pub fn command_start() -> Arc<PreMatcher<MessageEvent>> {
    Arc::new(command_start_)
}
