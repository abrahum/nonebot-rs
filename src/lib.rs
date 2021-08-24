mod api;
mod axum_driver;
mod bot;
pub mod butin;
mod config;
mod event;
mod log;
mod matcher;
mod message;
mod results;
mod utils;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

pub type MatchersVec<E> = Vec<matcher::Matcher<E>>;

#[derive(Clone)]
pub struct Matchers {
    message: MatchersVec<event::MessageEvent>,
    notice: MatchersVec<event::NoticeEvent>,
    request: MatchersVec<event::RequestEvent>,
    meta: MatchersVec<event::MetaEvent>,
}

fn unoption<T>(input: &Option<Vec<T>>) -> Vec<T>
where
    T: Clone,
{
    match input {
        Some(t) => t.clone(),
        None => vec![],
    }
}

fn singleclone<E>(input: &MatchersVec<E>) -> MatchersVec<E>
where
    E: Clone,
{
    let mut rvec = vec![];
    for handler in input {
        rvec.push(handler.clone());
    }
    rvec
}

impl Matchers {
    pub fn new(
        message: Option<MatchersVec<event::MessageEvent>>,
        notice: Option<MatchersVec<event::NoticeEvent>>,
        request: Option<MatchersVec<event::RequestEvent>>,
        meta: Option<MatchersVec<event::MetaEvent>>,
    ) -> Matchers {
        Matchers {
            message: unoption(&message),
            notice: unoption(&notice),
            request: unoption(&request),
            meta: unoption(&meta),
        }
    }

    pub fn clone(&self) -> Matchers {
        Matchers {
            message: singleclone(&self.message),
            notice: singleclone(&self.notice),
            request: singleclone(&self.request),
            meta: singleclone(&self.meta),
        }
    }
}

#[derive(Debug)]
pub struct Bot {
    superusers: Vec<String>,
    pub nickname: Vec<String>,
    command_start: Vec<String>,
    sender: Option<bot::ApiSender>,
}

pub struct Nonebot {
    // 作为根结构体，用来传递各种全局变量和设定值
    pub config: config::NbConfig, // 全局设置
    pub bots: HashMap<String, Bot>,
    pub matchers: Matchers,
}

impl Nonebot {
    fn build_bots(config: &config::NbConfig) -> HashMap<String, Bot> {
        let mut rmap = HashMap::new();
        if let Some(bots_config) = &config.bots {
            for (bot_id, bot_config) in bots_config {
                rmap.insert(
                    bot_id.clone(),
                    Bot {
                        sender: None,
                        superusers: unoption(&bot_config.superusers),
                        nickname: unoption(&bot_config.nickname),
                        command_start: unoption(&bot_config.command_start),
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
                    superusers: unoption(&self.config.global.superusers),
                    nickname: unoption(&self.config.global.nickname),
                    command_start: unoption(&self.config.global.command_start),
                },
            );
        }
    }

    pub fn new(matchers: Matchers) -> Self {
        let config = config::NbConfig::load();
        Nonebot {
            bots: Nonebot::build_bots(&config),
            config: config::NbConfig::load(),
            matchers: matchers,
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
