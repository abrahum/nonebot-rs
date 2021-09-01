use crate::config::BotConfig;
use crate::event::{Event, MessageEvent, MetaEvent, NoticeEvent, RequestEvent, SelfId};
use crate::log::log_load_matchers;
use crate::matcher::Matcher;
use crate::{ApiRespWatcher, ApiSender};
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
        }
    }

    pub fn get(&mut self, m: &Matchers) {
        self.message = m.message.clone();
        self.notice = m.notice.clone();
        self.request = m.request.clone();
        self.meta = m.meta.clone();
    }

    /// Bot 连接时运行所有 Matcher on_bot_connect 方法
    pub fn run_on_connect(
        &self,
        api_sender: crate::ApiSender,
        api_resp_watcher: crate::ApiRespWatcher,
    ) {
        fn run_on_connect_<E>(
            matcherb: &MatchersBTreeMap<E>,
            api_sender: crate::ApiSender,
            api_resp_watcher: crate::ApiRespWatcher,
        ) where
            E: Clone,
        {
            for (_, matcherh) in matcherb {
                for (_, matcher) in matcherh {
                    matcher
                        .build(api_sender.clone(), api_resp_watcher.clone())
                        .get_handler()
                        .on_bot_connect(matcher.clone());
                }
            }
        }

        log_load_matchers(&self);
        run_on_connect_(&self.message, api_sender.clone(), api_resp_watcher.clone());
        run_on_connect_(&self.notice, api_sender.clone(), api_resp_watcher.clone());
        run_on_connect_(&self.request, api_sender.clone(), api_resp_watcher.clone());
        run_on_connect_(&self.meta, api_sender.clone(), api_resp_watcher.clone());
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

    pub fn handle_events(
        &self,
        events: Event,
        config: BotConfig,
        api_sender: ApiSender,
        api_resp_watcher: ApiRespWatcher,
    ) {
        let mut matchers = self.clone();
        tokio::spawn(async move {
            match events {
                Event::Message(e) => {
                    Matchers::handle_event(
                        &mut matchers.message,
                        e,
                        config,
                        api_sender,
                        api_resp_watcher,
                    )
                    .await;
                }
                Event::Notice(e) => {
                    Matchers::handle_event(
                        &mut matchers.notice,
                        e,
                        config,
                        api_sender,
                        api_resp_watcher,
                    )
                    .await;
                }
                Event::Request(e) => {
                    Matchers::handle_event(
                        &mut matchers.request,
                        e,
                        config,
                        api_sender,
                        api_resp_watcher,
                    )
                    .await;
                }
                Event::Meta(e) => {
                    Matchers::handle_event(
                        &mut matchers.meta,
                        e,
                        config,
                        api_sender,
                        api_resp_watcher,
                    )
                    .await;
                }
            }
        });
    }

    /// 接收按类型分发后的 Event 逐级匹配 Matcher
    async fn handle_event<E>(
        matcherb: &mut crate::MatchersBTreeMap<E>,
        e: E,
        config: BotConfig,
        api_sender: ApiSender,
        api_resp_watcher: ApiRespWatcher,
    ) where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event {:?}", e);
        // 根据不同 Event 类型，逐级匹配，判定是否 Block
        for (_, matcherh) in matcherb.iter_mut() {
            if Matchers::handler_event_(
                matcherh,
                e.clone(),
                config.clone(),
                api_sender.clone(),
                api_resp_watcher.clone(),
            )
            .await
            {
                break;
            };
        }
    }

    #[doc(hidden)]
    #[cfg(feature = "matcher")]
    async fn handler_event_<E>(
        matcherh: &mut crate::MatchersHashMap<E>,
        e: E,
        config: BotConfig,
        api_sender: ApiSender,
        api_resp_watcher: ApiRespWatcher,
    ) -> bool
    where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event_ {:?}", e);
        // 每级 Matcher 匹配，返回是否 block
        let mut get_block = false;
        for (name, matcher) in matcherh.iter_mut() {
            let matched = matcher
                .build(api_sender.clone(), api_resp_watcher.clone())
                .match_(e.clone(), config.clone())
                .await;
            if matched {
                event!(Level::INFO, "Matched {}", name);
                if matcher.is_block() {
                    get_block = true;
                }
                if matcher.is_temp() {
                    matcher
                        .set(crate::Action::RemoveMatcher {
                            name: matcher.name.clone(),
                        })
                        .await;
                }
            }
        }
        get_block
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
