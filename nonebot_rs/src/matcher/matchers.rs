use crate::event::{Event, MessageEvent, MetaEvent, NoticeEvent, RequestEvent, SelfId};
use crate::log::log_load_matchers;
use crate::matcher::Matcher;
use std::collections::{BTreeMap, HashMap};
use tracing::{event, Level};

/// 按 `priority` 依序存储 `MatchersHashMap`
pub type MatchersBTreeMap<E> = BTreeMap<i8, MatchersHashMap<E>>;
/// 使用唯一名字存储 `Matcher`
pub type MatchersHashMap<E> = HashMap<String, Matcher<E>>;

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
    bot_getter: Option<crate::BotGettter>,
}

impl Matchers {
    /// 新建 Matchers
    pub fn new(
        message: Option<MatchersBTreeMap<MessageEvent>>,
        notice: Option<MatchersBTreeMap<NoticeEvent>>,
        request: Option<MatchersBTreeMap<RequestEvent>>,
        meta: Option<MatchersBTreeMap<MetaEvent>>,
    ) -> Matchers {
        Matchers {
            message: unoptionb(&message),
            notice: unoptionb(&notice),
            request: unoptionb(&request),
            meta: unoptionb(&meta),
            bot_getter: None,
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
    pub fn add_message_matcher(&mut self, matcher: Matcher<MessageEvent>) -> &mut Self {
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
        }
    }

    /// 接收按类型分发后的 Event 逐级匹配 Matcher
    async fn handle_event<E>(
        &mut self,
        mut matcherb: crate::MatchersBTreeMap<E>,
        event: E,
        bot: crate::bot::Bot,
    ) where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event {:?}", event);
        // 根据不同 Event 类型，逐级匹配，判定是否 Block
        for (_, matcherh) in matcherb.iter_mut() {
            if self
                ._handler_event_(matcherh, event.clone(), bot.clone())
                .await
            {
                break;
            };
        }
    }

    #[doc(hidden)]
    async fn _handler_event_<E>(
        &mut self,
        matcherh: &mut crate::MatchersHashMap<E>,
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
                event!(Level::INFO, "Matched {}", name);
                if matcher.is_block() {
                    get_block = true;
                }
                if matcher.is_temp() {
                    self.remove_matcher(&matcher.name);
                }
            }
        }
        get_block
    }

    async fn event_recv(mut self, mut event_receiver: crate::EventReceiver) {
        while let Ok(event) = event_receiver.recv().await {
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
}
