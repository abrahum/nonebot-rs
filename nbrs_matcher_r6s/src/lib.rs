use nonebot_rs::builtin::prematcher::*;
use nonebot_rs::event::MessageEvent;
use nonebot_rs::matcher::Matcher;
use reqwest::{header::HeaderMap, Client};
use std::sync::Arc;

mod base;
mod plays;
mod pro;
mod set;
mod utils;

pub fn r6s() -> Vec<Matcher<MessageEvent>> {
    let mut headers = HeaderMap::new();
    headers.insert("Host", "www.r6s.cn".parse().unwrap());
    headers.insert("referer", "https://www.r6s.cn".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
    let client = Arc::new(utils::R6sClient {
        client: Client::new(),
        headers: headers,
    });
    vec![
        Matcher::new(
            "r6s",
            base::R6s {
                client: client.clone(),
            },
        )
        .add_pre_matcher(to_me())
        .add_pre_matcher(command_start())
        .set_priority(3),
        Matcher::new(
            "r6spro",
            pro::R6sPro {
                client: client.clone(),
            },
        )
        .add_pre_matcher(to_me())
        .add_pre_matcher(command_start())
        .set_priority(1)
        .set_block(true),
        Matcher::new(
            "r6splays",
            plays::R6sPlays {
                client: client.clone(),
            },
        )
        .add_pre_matcher(to_me())
        .add_pre_matcher(command_start())
        .set_priority(2)
        .set_block(true),
        Matcher::new(
            "r6sset",
            set::R6sSet {
                client: client.clone(),
            },
        )
        .add_pre_matcher(to_me())
        .add_pre_matcher(command_start())
        .set_priority(2)
        .set_block(true),
    ]
}
