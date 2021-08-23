mod api;
mod axum_driver;
mod bot;
mod butin;
mod config;
mod event;
mod log;
mod message;
mod results;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

pub struct Bot {
    superusers: Vec<String>,
    nickname: Vec<String>,
    command_start: Vec<String>,
    sender: Option<bot::ApiSender>,
}

pub struct Nonebot {
    // 作为根结构体，用来传递各种全局变量和设定值
    // pub message_event_matchers: MatcherMap<event::MessageEvent>, // 按照优先级存储 Matcher
    // pub notice_event_matchers: MatcherMap<event::NoticeEvent>,   // 按照优先级存储 Matcher
    // pub request_event_matchers: MatcherMap<event::RequestEvent>, // 按照优先级存储 Matcher
    // pub meta_event_matchers: MatcherMap<event::MetaEvent>,       // 按照优先级存储 Matcher
    pub config: config::NbConfig,                                // 全局设置
    pub bots: HashMap<String, Bot>,
}

fn unbox(data: &Option<Vec<String>>) -> Vec<String> {
    if let Some(t) = data {
        t.clone()
    } else {
        vec![]
    }
}

// fn push_or_default<T>(i: Option<MatcherMap<T>>) -> MatcherMap<T> {
//     if let Some(m) = i {
//         m
//     } else {
//         vec![]
//     }
// }

impl Nonebot {
    fn build_bots(config: &config::NbConfig) -> HashMap<String, Bot> {
        let mut rmap = HashMap::new();
        if let Some(bots_config) = &config.bots {
            for (bot_id, bot_config) in bots_config {
                rmap.insert(
                    bot_id.clone(),
                    Bot {
                        sender: None,
                        superusers: unbox(&bot_config.superusers),
                        nickname: unbox(&bot_config.nickname),
                        command_start: unbox(&bot_config.command_start),
                    },
                );
            }
        }
        rmap
    }

    pub fn add_bot(&mut self, bot_id: i64, sender: bot::ApiSender) {
        let bot_id = bot_id.to_string();
        if let Some(bot) = self.bots.get_mut(&bot_id) {
            bot.sender = Some(sender);
        } else {
            self.bots.insert(
                bot_id,
                Bot {
                    sender: Some(sender),
                    superusers: unbox(&self.config.global.superusers),
                    nickname: unbox(&self.config.global.nickname),
                    command_start: unbox(&self.config.global.command_start),
                },
            );
        }
    }

    pub fn new(
        // message_event_matchers: Option<MatcherMap<event::MessageEvent>>,
        // notice_event_matchers: Option<MatcherMap<event::NoticeEvent>>,
        // request_event_matchers: Option<MatcherMap<event::RequestEvent>>,
        // meta_event_matchers: Option<MatcherMap<event::MetaEvent>>,
    ) -> Self {
        let config = config::NbConfig::load();
        Nonebot {
            // message_event_matchers: push_or_default(message_event_matchers),
            // notice_event_matchers: push_or_default(notice_event_matchers),
            // request_event_matchers: push_or_default(request_event_matchers),
            // meta_event_matchers: push_or_default(meta_event_matchers),
            bots: Nonebot::build_bots(&config),
            config: config::NbConfig::load(),
        }
    }
}

impl Nonebot {
    pub fn run(self) {
        log::init(self.config.global.debug);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(axum_driver::run(Arc::new(Mutex::new(self))));
    }
}
