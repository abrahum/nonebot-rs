mod api;
mod axum_driver;
mod bot;
pub mod builtin;
mod config;
mod event;
mod log;
mod matcher;
mod message;
mod results;
mod utils;

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

pub type MatchersBTreeMap<E> = BTreeMap<i8, MatchersHashMap<E>>;
pub type MatchersHashMap<E> = HashMap<String, matcher::Matcher<E>>;

#[derive(Clone)]
pub struct Matchers {
    message: MatchersBTreeMap<event::MessageEvent>,
    notice: MatchersBTreeMap<event::NoticeEvent>,
    request: MatchersBTreeMap<event::RequestEvent>,
    meta: MatchersBTreeMap<event::MetaEvent>,
}

fn unoptionb<K, D>(input: &Option<BTreeMap<K, D>>) -> BTreeMap<K, D>
where
    K: Clone + std::cmp::Ord,
    D: Clone,
{
    match input {
        Some(t) => t.clone(),
        None => BTreeMap::new(),
    }
}

impl Matchers {
    pub fn new(
        message: Option<MatchersBTreeMap<event::MessageEvent>>,
        notice: Option<MatchersBTreeMap<event::NoticeEvent>>,
        request: Option<MatchersBTreeMap<event::RequestEvent>>,
        meta: Option<MatchersBTreeMap<event::MetaEvent>>,
    ) -> Matchers {
        Matchers {
            message: unoptionb(&message),
            notice: unoptionb(&notice),
            request: unoptionb(&request),
            meta: unoptionb(&meta),
        }
    }

    pub fn add_message_matcher(
        &mut self,
        matcher: matcher::Matcher<event::MessageEvent>,
    ) -> Matchers {
        match self.message.get(&matcher.priority) {
            Some(_) => {
                self.message
                    .get_mut(&matcher.priority)
                    .unwrap()
                    .insert(matcher.name.clone(), matcher)
                    .unwrap();
            }
            None => {
                let mut hashmap: MatchersHashMap<event::MessageEvent> = HashMap::new();
                hashmap.insert(matcher.name.clone(), matcher.clone());
                self.message.insert(matcher.priority, hashmap);
            }
        }
        self.clone()
    }

    fn set_sender_<E>(matcherb: &mut MatchersBTreeMap<E>, sender: bot::ApiSender)
    where
        E: Clone,
    {
        for (_, matcherh) in matcherb.iter_mut() {
            for (_, matcher) in matcherh.iter_mut() {
                matcher.set_sender(sender.clone());
            }
        }
    }

    pub fn set_sender(&mut self, sender: bot::ApiSender) -> Matchers {
        Matchers::set_sender_(&mut self.message, sender.clone());
        Matchers::set_sender_(&mut self.notice, sender.clone());
        Matchers::set_sender_(&mut self.request, sender.clone());
        Matchers::set_sender_(&mut self.meta, sender.clone());
        self.clone()
    }
}

#[derive(Debug)]
pub struct Bot {
    pub superusers: Vec<String>,
    pub nickname: Vec<String>,
    pub command_start: Vec<String>,
    pub sender: Option<bot::ApiSender>,
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
                        superusers: bot_config.superusers.clone(),
                        nickname: bot_config.nickname.clone(),
                        command_start: bot_config.command_start.clone(),
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
                    superusers: self.config.global.superusers.clone(),
                    nickname: self.config.global.nickname.clone(),
                    command_start: self.config.global.command_start.clone(),
                },
            );
        }
    }

    pub fn new() -> Self {
        let config = config::NbConfig::load();
        Nonebot {
            bots: Nonebot::build_bots(&config),
            config: config::NbConfig::load(),
            matchers: Matchers::new(None, None, None, None),
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
