use crate::event::MessageEvent;
use crate::matcher::{AMNb, PreMatcher};
use crate::message::Message;
use crate::utils::remove_space;
use std::sync::Arc;

pub fn to_me() -> Arc<PreMatcher<MessageEvent>> {
    let to_me = |e: &MessageEvent, amnb: AMNb| -> Option<MessageEvent> {
        match e {
            MessageEvent::Private(_) => Some(e.clone()),
            MessageEvent::Group(g) => {
                let nickname: Vec<String>;
                let bot_id = g.self_id.to_string();
                {
                    let nb = amnb.lock().unwrap();
                    nickname = nb.bots.get(&bot_id).unwrap().nickname.clone();
                };
                let raw_message = remove_space(&g.raw_message);
                for name in nickname {
                    if raw_message.starts_with(&name) {
                        let mut rg = g.clone();
                        rg.raw_message = remove_space(&raw_message[name.len()..]);
                        return Some(MessageEvent::Group(rg));
                    }
                }
                for message in &g.message {
                    match message {
                        Message::At(at) => {
                            if at.qq == bot_id {
                                return Some(MessageEvent::Group(g.clone()));
                            }
                        }
                        _ => continue,
                    }
                }
                None
            }
        }
    };
    Arc::new(to_me)
}

pub fn command_start(event: &MessageEvent, amnb: AMNb) -> Option<MessageEvent> {
    let raw_message = remove_space(&event.get_raw_message());
    let command_starts: Vec<String>;
    {
        let nb = amnb.lock().unwrap();
        let bot = nb.bots.get(&event.get_self_id()).unwrap();
        command_starts = bot.command_start.clone()
    }
    if command_starts.is_empty() {
        return Some(event.clone());
    }
    for sc in &command_starts {
        if raw_message.starts_with(sc) {
            let new_raw_message = remove_space(&raw_message[sc.len()..]);
            let new_messageevent = event.set_raw_message(new_raw_message);
            return Some(new_messageevent);
        }
    }
    None
}
