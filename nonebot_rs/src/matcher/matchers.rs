use crate::event::{Event, MessageEvent, MetaEvent, NoticeEvent, RequestEvent, SelfId};
use crate::matcher::Matcher;
use colored::*;
use std::collections::{BTreeMap, HashMap};
use tokio::sync::broadcast;
use tracing::{event, Level};

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
    bot_getter: Option<crate::BotGettter>,
    /// Matchers Action Sender
    action_sender: ActionSender,
    /// Config
    config: HashMap<String, HashMap<String, toml::Value>>,
}

impl Matchers {
    /// 新建 Matchers
    pub fn new(
        message: Option<MatchersBTreeMap<MessageEvent>>,
        notice: Option<MatchersBTreeMap<NoticeEvent>>,
        request: Option<MatchersBTreeMap<RequestEvent>>,
        meta: Option<MatchersBTreeMap<MetaEvent>>,
    ) -> Matchers {
        let (sender, _) = broadcast::channel(32);
        Matchers {
            message: unoptionb(&message),
            notice: unoptionb(&notice),
            request: unoptionb(&request),
            meta: unoptionb(&meta),
            bot_getter: None,
            action_sender: sender,
            config: HashMap::new(),
        }
    }

    /// 新建空 Matchers
    pub fn new_empty() -> Matchers {
        Matchers::new(None, None, None, None)
    }

    pub fn get(&mut self, m: &Matchers) {
        self.message = m.message.clone();
        self.notice = m.notice.clone();
        self.request = m.request.clone();
        self.meta = m.meta.clone();
    }

    /// Bot 连接时运行所有 Matcher on_bot_connect 方法
    pub fn run_on_connect(&self, bot: crate::bot::Bot) {
        fn run_on_connect_<E>(matcherb: &MatchersBTreeMap<E>, bot: crate::bot::Bot)
        where
            E: Clone,
        {
            for (_, matcherh) in matcherb {
                for (_, matcher) in matcherh {
                    matcher
                        .build(bot.clone())
                        .get_handler()
                        .on_bot_connect(matcher.clone());
                }
            }
        }

        log_load_matchers(&self);
        run_on_connect_(&self.message, bot.clone());
        run_on_connect_(&self.notice, bot.clone());
        run_on_connect_(&self.request, bot.clone());
        run_on_connect_(&self.meta, bot.clone());
    }

    /// 向 Matchers 添加 Matcher<MessageEvent>
    pub fn add_message_matcher(&mut self, mut matcher: Matcher<MessageEvent>) -> &mut Self {
        matcher.set_action_sender(self.action_sender.clone());
        match self.message.get(&matcher.priority) {
            Some(_) => {
                self.message
                    .get_mut(&matcher.priority)
                    .unwrap()
                    .insert(matcher.name.clone(), matcher);
            }
            None => {
                let mut hashmap: MatchersHashMap<MessageEvent> = HashMap::new();
                hashmap.insert(matcher.name.clone(), matcher.clone());
                self.message.insert(matcher.priority, hashmap);
            }
        }
        self
    }

    /// 向 Matchers 添加 Vec<Matcher<MessageEvent>>
    pub fn add_message_matchers(&mut self, matchers: Vec<Matcher<MessageEvent>>) -> &mut Self {
        for m in matchers {
            self.add_message_matcher(m);
        }
        self
    }

    /// 根据 Matcher.name 从 Matchers 移除 Matcher
    pub fn remove_matcher(&mut self, name: &str) {
        fn remove_matcher_<E>(matcherb: &mut MatchersBTreeMap<E>, name: &str)
        where
            E: Clone,
        {
            for (_, matcherh) in matcherb.iter_mut() {
                if let Some(_) = matcherh.remove(name) {
                    return;
                }
            }
        }

        remove_matcher_(&mut self.message, name);
        remove_matcher_(&mut self.notice, name);
        remove_matcher_(&mut self.request, name);
        remove_matcher_(&mut self.meta, name);
    }

    /// 根据 Matcher.name disable Matcher
    pub fn disable_matcher(&mut self, name: &str, disable: bool) {
        fn disable_matcher_<E>(matcherb: &mut MatchersBTreeMap<E>, name: &str, disable: bool)
        where
            E: Clone,
        {
            for (_, matcherh) in matcherb.iter_mut() {
                if let Some(matcher) = matcherh.get_mut(name) {
                    matcher.set_disable(disable);
                }
            }
        }

        disable_matcher_(&mut self.message, name, disable);
        disable_matcher_(&mut self.notice, name, disable);
        disable_matcher_(&mut self.request, name, disable);
        disable_matcher_(&mut self.meta, name, disable);
    }

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
                crate::event::NbEvent::BotConnect { bot } => self.run_on_connect(bot),
                crate::event::NbEvent::BotDisconnect { bot: _ } => {}
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

#[doc(hidden)]
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

impl crate::Plugin for Matchers {
    fn run(&self, event_receiver: crate::EventReceiver, bot_getter: crate::BotGettter) {
        let mut m = self.clone();
        m.bot_getter = Some(bot_getter.clone());
        tokio::spawn(m.event_recv(event_receiver));
    }

    fn plugin_name(&self) -> &'static str {
        PLUGIN_NAME
    }

    fn load_config(&mut self, config: toml::Value) {
        let config: HashMap<String, HashMap<String, toml::Value>> =
            config.try_into().expect("Matchers get error config");
        self.config = config;
        event!(Level::INFO, "Loaded Matchers config: {:?}", self.config);
    }
}

pub fn log_load_matchers(matchers: &crate::Matchers) {
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
