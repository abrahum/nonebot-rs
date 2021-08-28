use crate::api::Apis;
use crate::builtin;
use crate::config::BotConfig;
use crate::event::{Events, SelfId};
use crate::log::log_load_matchers;
use crate::{Matchers, Nonebot};
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast::Receiver, mpsc::Sender};
use tracing::{event, Level};

/// 发送 Api 请求的 mpsc channel sender
pub type ApiSender = Sender<ChannelItem>;

/// Bot 运行实例
pub struct Bot {
    self_id: String, // bot ID
    listener: Receiver<Setter>,
    // amnb: Arc<Mutex<Nonebot>>, // Nonebot
    sender: ApiSender,  // channel sender
    matchers: Matchers, // Bot Matchers
    config: BotConfig,  // Bot config
}

impl Bot {
    /// 新建 Bot 实例
    pub fn new(
        id: i64,
        authorization: Option<String>,
        sender: ApiSender,
        listener: Receiver<Setter>,
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
            listener: listener,
            // amnb: amnb,
            sender: sender,
            matchers: matchers,
            config: config,
        };
        log_load_matchers(&bot.matchers);
        bot.matchers.run_on_connect();
        Ok(bot)
    }

    /// 返回 `Bot.self_id` 属性
    #[allow(dead_code)]
    pub fn get_self_id(&self) -> &str {
        &self.self_id
    }

    /// 接收 WebSocket 消息处理并分发 Event 和 ApiResp
    pub async fn handle_recv(&mut self, msg: String) {
        // 处理接收到所有消息，分流上报 Event 和 Api 调用回执
        while let Ok(set) = self.listener.try_recv() {
            event!(Level::DEBUG, "get set {:?}", set);
            self.handle_setter(set);
        }
        let data: serde_json::error::Result<Events> = serde_json::from_str(&msg);
        match data {
            Ok(events) => self.handle_events(events).await,
            Err(_) => self.handle_resp(msg).await,
        }
    }

    /// 接收 Setter 并处理
    fn handle_setter(&mut self, set: Setter) {
        match set {
            Setter::AddMessageEventMatcher {
                bot_id: id,
                message_event_matcher: mut matcher,
            } => {
                if id == self.self_id {
                    matcher.set_sender(self.sender.clone());
                    self.matchers.add_message_matcher(matcher.clone());
                    event!(
                        Level::DEBUG,
                        "[{}] Add temp matcher {:?}",
                        self.self_id,
                        matcher
                    );
                }
            }
            Setter::RemoveMatcher {
                bot_id: id,
                name: matcher_name,
            } => {
                if id == self.self_id {
                    self.matchers.remove_matcher(&matcher_name);
                    event!(
                        Level::DEBUG,
                        "[{}] Remove matcher {}",
                        self.self_id,
                        matcher_name
                    );
                }
            }
            Setter::DisableMatcher {
                bot_id: id,
                name: matcher_name,
            } => {
                if id == self.self_id {
                    self.matchers.disable_matcher(&matcher_name, true);
                    event!(
                        Level::DEBUG,
                        "[{}] Disable matcher{}",
                        self.self_id,
                        matcher_name
                    );
                }
            }
            Setter::EnableMatcher {
                bot_id: id,
                name: matcher_name,
            } => {
                if id == self.self_id {
                    self.matchers.disable_matcher(&matcher_name, false);
                    event!(
                        Level::DEBUG,
                        "[{}] Enable matcher{}",
                        self.self_id,
                        matcher_name
                    );
                }
            }
            _ => {}
        }
    }

    /// 接受 Event ，根据 Event 类型分发（协程处理）
    async fn handle_events(&self, events: Events) {
        event!(Level::TRACE, "handling events {:?}", events);
        // 处理上报 Event 分流不同 Event 类型
        let matchers = self.matchers.clone();
        let config = self.config.clone();
        let bot_id = self.self_id.clone();
        tokio::spawn(async move {
            match events {
                Events::Message(e) => {
                    builtin::logger(&e).await.unwrap();
                    Bot::handle_event(&matchers.message, e, config, bot_id).await;
                }
                Events::Notice(e) => {
                    Bot::handle_event(&matchers.notice, e, config, bot_id).await;
                }
                Events::Request(e) => Bot::handle_event(&matchers.request, e, config, bot_id).await,
                Events::Meta(e) => {
                    builtin::metahandle(&e).await;
                    Bot::handle_event(&matchers.meta, e, config, bot_id).await;
                }
            }
        });
    }

    /// 接收 ApiResp 处理 log
    async fn handle_resp(&self, resp: String) {
        event!(Level::DEBUG, "handling resp {}", resp);
        // 处理 Api 调用回执
        let resp: crate::api::ApiResp = serde_json::from_str(&resp).unwrap();
        builtin::resp_logger(resp).await;
    }

    /// 接收按类型分发后的 Event 逐级匹配 Matcher
    async fn handle_event<E>(
        matcherb: &crate::MatchersBTreeMap<E>,
        e: E,
        config: BotConfig,
        bot_id: String,
    ) where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event {:?}", e);
        // 根据不同 Event 类型，逐级匹配，判定是否 Block
        for (_, matcherh) in matcherb {
            if Bot::handler_event_(matcherh, e.clone(), config.clone(), bot_id.clone()).await {
                break;
            };
        }
    }

    #[doc(hidden)]
    async fn handler_event_<E>(
        matcherh: &crate::MatchersHashMap<E>,
        e: E,
        config: BotConfig,
        bot_id: String,
    ) -> bool
    where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event_ {:?}", e);
        // 每级 Matcher 匹配，返回是否 block
        let mut get_block = false;
        for (_, matcher) in matcherh {
            let matched = matcher.match_(e.clone(), config.clone()).await;
            if matched && matcher.is_block() {
                get_block = true;
            }
            if matched && matcher.is_temp() {
                matcher
                    .set(Setter::RemoveMatcher {
                        bot_id: bot_id.clone(),
                        name: matcher.name.clone(),
                    })
                    .await;
            }
        }
        get_block
    }

    /// 连接鉴权
    fn check_auth(_auth: Option<String>, _amnb: Arc<Mutex<Nonebot>>) -> Result<bool, String> {
        // todo
        Ok(true)
    }
}

/// mpsc channel 传递项
#[derive(Debug)]
pub enum ChannelItem {
    /// Bot 内部设置项
    ///
    /// 接收后会由 broadcaster 分发给所有 Bot
    Setter(Setter),
    /// Onebot Api
    Apis(Apis),
}

/// Bot 内部设置项
#[derive(Debug, Clone)]
pub enum Setter {
    /// 移除 Matcher
    RemoveMatcher { bot_id: String, name: String },
    /// 添加 Matcher<MessageEvent>
    AddMessageEventMatcher {
        bot_id: String,
        message_event_matcher: crate::matcher::Matcher<crate::event::MessageEvent>,
    },
    /// 禁用 Matcher
    DisableMatcher { bot_id: String, name: String },
    /// 取消禁用 Matcher
    EnableMatcher { bot_id: String, name: String },
    /// 变更 BotConfig
    ChangeBotConfig {
        bot_id: String,
        bot_config: BotConfig,
    },
}