pub mod echo;
pub mod echo_;

use crate::event::{MessageEvent, MetaEvent};
// use crate::matcher::AMNb;
use crate::results::HandlerResult;
use colored::*;
use tracing::{event, Level};

pub async fn logger(event: MessageEvent) -> HandlerResult {
    match &event {
        MessageEvent::Private(p) => {
            event!(
                Level::INFO,
                "Bot {} receive -> {} from {}({})",
                p.self_id.to_string().red(),
                p.raw_message,
                p.sender.nickname.to_string().blue(),
                p.user_id.to_string().green(),
            )
        }
        MessageEvent::Group(g) => {
            event!(
                Level::INFO,
                "Bot {} receive in group {} -> {} from {}({})",
                g.self_id.to_string().red(),
                g.group_id.to_string().magenta(),
                g.raw_message,
                g.sender.nickname.to_string().blue(),
                g.user_id.to_string().green(),
            )
        }
    }
    Ok(true)
}

pub async fn resp_logger(resp: crate::api::ApiResp) {
    if &resp.status == "ok" {
        event!(Level::TRACE, "{} success", resp.echo);
    } else {
        event!(Level::INFO, "{} failed", resp.echo);
    }
}

pub async fn metahandle(event: MetaEvent) {
    if &event.meta_event_type == "heartbeat" {
        event!(Level::TRACE, "Recive HeartBeat")
    }
}
