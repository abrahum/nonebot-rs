use crate::api::Apis;
use crate::builtin;
use crate::config::BotConfig;
use crate::event::Events;
use crate::log::log_load_matchers;
use crate::{Matchers, Nonebot};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;
use tracing::{event, Level};

pub type ApiSender = Sender<Apis>;

pub struct Bot {
    self_id: String, // bot ID
    // amnb: Arc<Mutex<Nonebot>>, // Nonebot
    sender: ApiSender,  // channel sender
    matchers: Matchers, // Bot Matchers
    config: BotConfig,  // Bot config
}

impl Bot {
    pub fn new(
        id: i64,
        authorization: Option<String>,
        sender: Sender<Apis>,
        amnb: Arc<Mutex<Nonebot>>,
    ) -> Result<Self, String> {
        Bot::check_auth(authorization, amnb.clone())?;
        let mut matchers: Matchers;
        let config: BotConfig;
        {
            let nb = amnb.lock().unwrap();
            matchers = nb.matchers.clone();
            if let Some(bots_config) = &nb.config.bots {
                if let Some(bot_config) = bots_config.get(&id.to_string()) {
                    config = bot_config.clone();
                } else {
                    config = BotConfig {
                        superusers: nb.config.global.superusers.clone(),
                        nickname: nb.config.global.nickname.clone(),
                        command_start: nb.config.global.command_start.clone(),
                    }
                }
            } else {
                config = BotConfig {
                    superusers: nb.config.global.superusers.clone(),
                    nickname: nb.config.global.nickname.clone(),
                    command_start: nb.config.global.command_start.clone(),
                }
            }
        }
        matchers.set_sender(sender.clone());
        let bot = Bot {
            self_id: id.to_string(),
            // amnb: amnb,
            sender: sender,
            matchers: matchers,
            config: config,
        };
        log_load_matchers(&bot.matchers);
        Ok(bot)
    }

    #[allow(dead_code)]
    pub fn get_self_id(&self) -> &str {
        &self.self_id
    }

    pub async fn handle_recv(&self, msg: String) {
        // 处理接收到所有消息，分流上报 Event 和 Api 调用回执
        let data: serde_json::error::Result<Events> = serde_json::from_str(&msg);
        match data {
            Ok(events) => self.handle_events(events).await,
            Err(_) => self.handle_resp(msg).await,
        }
        // match self.sender.try_send(crate::api::Apis::None) {
        //     Ok(_) => {}
        //     Err(_) => {}
        // };
    }

    async fn handle_events(&self, events: Events) {
        event!(Level::TRACE, "handling events {:?}", events);
        // 处理上报 Event 分流不同 Event 类型
        match events {
            Events::Message(e) => {
                builtin::logger(&e).await.unwrap();
                self.handle_event(&self.matchers.message, e).await;
            }
            Events::Notice(e) => {
                self.handle_event(&self.matchers.notice, e).await;
            }
            Events::Request(e) => self.handle_event(&self.matchers.request, e).await,
            Events::Meta(e) => {
                builtin::metahandle(&e).await;
                self.handle_event(&self.matchers.meta, e).await;
            }
        }
    }

    async fn handle_resp(&self, resp: String) {
        event!(Level::TRACE, "handling resp {}", resp);
        // 处理 Api 调用回执
        let resp: crate::api::ApiResp = serde_json::from_str(&resp).unwrap();
        builtin::resp_logger(resp).await;
    }

    async fn handle_event<E>(&self, matcherb: &crate::MatchersBTreeMap<E>, e: E)
    where
        E: Clone + Send + 'static + std::fmt::Debug,
    {
        event!(Level::TRACE, "handling event {:?}", e);
        // 根据不同 Event 类型，逐级匹配，判定是否 Block
        for (_, matcherh) in matcherb {
            if self.handler_event_(matcherh, e.clone()).await {
                break;
            };
        }
    }

    async fn handler_event_<E>(&self, matcherh: &crate::MatchersHashMap<E>, e: E) -> bool
    where
        E: Clone + Send + 'static + std::fmt::Debug,
    {
        event!(Level::TRACE, "handling event_ {:?}", e);
        // 每级 Matcher 匹配，返回是否 block
        let mut get_block = false;
        for (_, matcher) in matcherh {
            let matched = matcher.match_(e.clone(), self.config.clone()).await;
            if matched && matcher.is_block() {
                get_block = true;
            }
        }
        get_block
    }

    fn check_auth(auth: Option<String>, amnb: Arc<Mutex<Nonebot>>) -> Result<bool, String> {
        // todo 鉴权
        Ok(true)
    }
}
