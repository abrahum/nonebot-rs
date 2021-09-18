use crate::event::{Event, MessageEvent, MetaEvent, NoticeEvent, RequestEvent, SelfId};
use crate::matcher::Matcher;
use async_trait::async_trait;
use colored::*;
use std::collections::{BTreeMap, HashMap};
use tokio::sync::broadcast;
use tracing::{event, Level};

mod action;

/// 按 `priority` 依序存储 `MatchersHashMap`
pub type MatchersBTreeMap<E> = BTreeMap<i8, MatchersHashMap<E>>;
/// 使用唯一名字存储 `Matcher`
pub type MatchersHashMap<E> = HashMap<String, Matcher<E>>;
/// Matchers Action Sender
pub type ActionSender = broadcast::Sender<super::action::MatchersAction>;

pub const PLUGIN_NAME: &'static str = "Matcher";

/// 根据 `Event` 类型分类存储对应的 `Matcher`
#[derive(Clone, Debug)]
pub struct Matchers {
    /// MessageEvent 对应 MatcherBTreeMap
    pub message: MatchersBTreeMap<MessageEvent>,
    /// NoticeEvent 对应 MatcherBTreeMap
    pub notice: MatchersBTreeMap<NoticeEvent>,
    /// RequestEvent 对应 MatcherBTreeMap
    pub request: MatchersBTreeMap<RequestEvent>,
    /// MetaEvent 对应 MatcherBTreeMap
    pub meta: MatchersBTreeMap<MetaEvent>,
    /// Bot Watch channel Receiver
    bot_getter: Option<crate::BotGetter>,
    /// Matchers Action Sender
    action_sender: ActionSender,
    /// Config
    config: HashMap<String, HashMap<String, toml::Value>>,
}

impl Matchers {
    async fn handle_events(&mut self, event: Event, bot: &crate::bot::Bot) {
        match event {
            Event::Message(e) => {
                self.handle_event(self.message.clone(), e, bot.clone())
                    .await;
            }
            Event::Notice(e) => {
                self.handle_event(self.notice.clone(), e, bot.clone()).await;
            }
            Event::Request(e) => {
                self.handle_event(self.request.clone(), e, bot.clone())
                    .await;
            }
            Event::Meta(e) => {
                self.handle_event(self.meta.clone(), e, bot.clone()).await;
            }
            Event::Nonebot(e) => match e {
                crate::event::NbEvent::BotConnect { bot } => {
                    log_load_matchers(&self);
                    self.run_on_connect(bot, false).await;
                }
                crate::event::NbEvent::BotDisconnect { bot } => {
                    self.run_on_connect(bot, true).await;
                }
            },
        }
    }

    /// 接收按类型分发后的 Event 逐级匹配 Matcher
    async fn handle_event<E>(
        &mut self,
        mut matcherb: MatchersBTreeMap<E>,
        event: E,
        bot: crate::bot::Bot,
    ) where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event {:?}", event);
        // 根据不同 Event 类型，逐级匹配，判定是否 Block
        for (_, matcherh) in matcherb.iter_mut() {
            if self
                ._handler_event(matcherh, event.clone(), bot.clone())
                .await
            {
                break;
            };
        }
    }

    #[doc(hidden)]
    async fn _handler_event<E>(
        &mut self,
        matcherh: &mut MatchersHashMap<E>,
        e: E,
        bot: crate::bot::Bot,
    ) -> bool
    where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event_ {:?}", e);
        // 每级 Matcher 匹配，返回是否 block
        let mut get_block = false;
        let config = bot.config.clone();
        for (name, matcher) in matcherh.iter_mut() {
            let matched = matcher
                .build(bot.clone())
                .match_(e.clone(), config.clone(), self)
                .await;
            if matched {
                event!(Level::INFO, "Matched {}", name.blue());
                if matcher.is_block() {
                    get_block = true;
                }
                if matcher.is_temp() {
                    event!(Level::INFO, "Remove matched temp matcher {}", name.blue());
                    self.remove_matcher(name);
                }
            }
        }
        get_block
    }

    async fn event_recv(mut self, mut event_receiver: crate::EventReceiver) {
        let mut receiver = self.action_sender.subscribe();
        while let Ok(event) = event_receiver.recv().await {

            match receiver.try_recv() {
                Ok(action) => self.handle_action(action),
                Err(_) => {}
            }

            let bots = self.bot_getter.clone().unwrap().borrow().clone();
            if let Some(bot) = bots.get(&event.get_self_id()) {
                self.handle_events(event, bot).await;
            }
        }
    }
}

#[async_trait]
impl crate::Plugin for Matchers {
    fn run(&self, event_receiver: crate::EventReceiver, bot_getter: crate::BotGetter) {
        let mut m = self.clone();
        m.bot_getter = Some(bot_getter.clone());
        tokio::spawn(m.event_recv(event_receiver));
    }

    fn plugin_name(&self) -> &'static str {
        PLUGIN_NAME
    }

    async fn load_config(&mut self, config: toml::Value) {
        let config: HashMap<String, HashMap<String, toml::Value>> =
            config.try_into().expect("Matchers get error config");
        self.config = config;
        self.load_all_matcher_config().await;
        event!(Level::INFO, "Loaded Matchers config: {:?}", self.config);
    }
}

fn log_load_matchers(matchers: &crate::Matchers) {
    log_matcherb(&matchers.message);
    log_matcherb(&matchers.notice);
    log_matcherb(&matchers.request);
    log_matcherb(&matchers.meta);
}

fn log_matcherb<E>(matcherb: &MatchersBTreeMap<E>)
where
    E: Clone,
{
    if matcherb.is_empty() {
        return;
    }
    for (_, matcherh) in matcherb {
        for (name, _) in matcherh {
            event!(Level::INFO, "Matcher {} is Loaded", name.blue());
        }
    }
}
