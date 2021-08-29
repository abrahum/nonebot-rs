use crate::bot::{ApiRespWatcher, ApiSender};
use crate::event;
use crate::matcher::Matcher;
use std::collections::{BTreeMap, HashMap};

/// 按 `priority` 依序存储 `MatchersHashMap`
pub type MatchersBTreeMap<E> = BTreeMap<i8, MatchersHashMap<E>>;
/// 使用唯一名字存储 `Matcher`
pub type MatchersHashMap<E> = HashMap<String, Matcher<E>>;

/// 根据 `Event` 类型分类存储对应的 `Matcher`
#[derive(Clone, Debug)]
pub struct Matchers {
    pub message: MatchersBTreeMap<event::MessageEvent>,
    pub notice: MatchersBTreeMap<event::NoticeEvent>,
    pub request: MatchersBTreeMap<event::RequestEvent>,
    pub meta: MatchersBTreeMap<event::MetaEvent>,
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

    pub fn run_on_connect(&self) {
        fn run_on_connect_<E>(matcherb: &MatchersBTreeMap<E>)
        where
            E: Clone,
        {
            for (_, matcherh) in matcherb {
                for (_, matcher) in matcherh {
                    matcher.get_handler().on_bot_connect(matcher.clone());
                }
            }
        }

        run_on_connect_(&self.message);
        run_on_connect_(&self.notice);
        run_on_connect_(&self.request);
        run_on_connect_(&self.meta);
    }

    pub fn add_message_matcher(&mut self, matcher: Matcher<event::MessageEvent>) -> &mut Self {
        match self.message.get(&matcher.priority) {
            Some(_) => {
                self.message
                    .get_mut(&matcher.priority)
                    .unwrap()
                    .insert(matcher.name.clone(), matcher);
            }
            None => {
                let mut hashmap: MatchersHashMap<event::MessageEvent> = HashMap::new();
                hashmap.insert(matcher.name.clone(), matcher.clone());
                self.message.insert(matcher.priority, hashmap);
            }
        }
        self
    }

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

    pub fn set_sender(&mut self, sender: ApiSender, watcher: ApiRespWatcher) {
        fn set_sender_<E>(
            matcherb: &mut MatchersBTreeMap<E>,
            sender: ApiSender,
            watcher: ApiRespWatcher,
        ) where
            E: Clone,
        {
            for (_, matcherh) in matcherb.iter_mut() {
                for (_, matcher) in matcherh.iter_mut() {
                    matcher.set_sender(sender.clone());
                    matcher.set_watcher(watcher.clone());
                }
            }
        }

        set_sender_(&mut self.message, sender.clone(), watcher.clone());
        set_sender_(&mut self.notice, sender.clone(), watcher.clone());
        set_sender_(&mut self.request, sender.clone(), watcher.clone());
        set_sender_(&mut self.meta, sender.clone(), watcher.clone());
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
