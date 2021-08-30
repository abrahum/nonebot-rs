/// 内建 echo Matcher
pub mod echo;
#[doc(hidden)]
pub mod macros;
/// 内建 PreMatcher 函数
pub mod prematchers;
/// rcnb！！！
pub mod rcnb;
/// 内建 rules
pub mod rules;

use crate::event::{MessageEvent, MetaEvent};
use crate::results::HandlerResult;
use colored::*;
use tracing::{event, Level};

#[doc(hidden)]
pub async fn logger(event: &MessageEvent) -> HandlerResult {
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
    Ok(true)
}

#[doc(hidden)]
pub fn resp_logger(resp: &crate::api_resp::ApiResp) {
    if &resp.status == "ok" {
        event!(Level::DEBUG, "{} success", resp.echo);
    } else {
        event!(Level::INFO, "{} failed", resp.echo);
    }
}

#[doc(hidden)]
pub async fn metahandle(event: &MetaEvent) {
    if &event.meta_event_type == "heartbeat" {
        event!(Level::TRACE, "Recive HeartBeat")
    }
}
