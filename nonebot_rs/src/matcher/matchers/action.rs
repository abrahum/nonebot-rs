use super::{Matchers, MatchersBTreeMap, MatchersHashMap};
use crate::event::{MessageEvent, MetaEvent, NoticeEvent, RequestEvent};
use crate::matcher::{action::MatchersAction, Matcher};
use std::collections::{BTreeMap, HashMap};
use tokio::sync::broadcast;

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
    pub async fn run_on_connect(&self, bot: crate::bot::Bot, disconnect: bool) {
        async fn run_on_connect_<E>(
            matcherb: &MatchersBTreeMap<E>,
            bot: crate::bot::Bot,
            disconnect: bool,
        ) where
            E: Clone,
        {
            for (_, matcherh) in matcherb {
                for (_, matcher) in matcherh {
                    let built_matcher = matcher.build(bot.clone());
                    let handler = built_matcher.get_handler();
                    let lock_handler = handler.read().await;
                    if disconnect {
                        lock_handler.on_bot_disconnect(matcher.clone());
                    } else {
                        lock_handler.on_bot_connect(matcher.clone());
                    }
                }
            }
        }

        run_on_connect_(&self.message, bot.clone(), disconnect).await;
        run_on_connect_(&self.notice, bot.clone(), disconnect).await;
        run_on_connect_(&self.request, bot.clone(), disconnect).await;
        run_on_connect_(&self.meta, bot.clone(), disconnect).await;
    }

    pub async fn load_all_matcher_config(&self) {
        async fn f<E>(
            matcherb: &MatchersBTreeMap<E>,
            config: &HashMap<String, HashMap<String, toml::Value>>,
        ) where
            E: Clone,
        {
            for (_, matcherh) in matcherb {
                for (matcher_name, matcher) in matcherh {
                    if let Some(data) = config.get(&matcher_name.to_lowercase()) {
                        let handler = matcher.get_handler();
                        let mut lock_handler = handler.write().await;
                        lock_handler.load_config(data.clone());
                    }
                }
            }
        }

        f(&self.message, &self.config).await;
        f(&self.notice, &self.config).await;
        f(&self.request, &self.config).await;
        f(&self.meta, &self.config).await;
    }

    fn add_matcher<E>(
        matcherb: &mut MatchersBTreeMap<E>,
        mut matcher: Matcher<E>,
        action_sender: broadcast::Sender<MatchersAction>,
    ) where
        E: Clone,
    {
        matcher.set_action_sender(action_sender);
        match matcherb.get_mut(&matcher.priority) {
            Some(h) => {
                h.insert(matcher.name.clone(), matcher);
            }
            None => {
                let mut hashmap: MatchersHashMap<E> = HashMap::new();
                hashmap.insert(matcher.name.clone(), matcher.clone());
                matcherb.insert(matcher.priority, hashmap);
            }
        }
    }

    /// 向 Matchers 添加 Matcher<MessageEvent>
    pub fn add_message_matcher(&mut self, matcher: Matcher<MessageEvent>) -> &mut Self {
        Matchers::add_matcher(&mut self.message, matcher, self.action_sender.clone());
        self
    }

    /// 向 Matchers 添加 Vec<Matcher<MessageEvent>>
    pub fn add_message_matchers(&mut self, matchers: Vec<Matcher<MessageEvent>>) -> &mut Self {
        for m in matchers {
            self.add_message_matcher(m);
        }
        self
    }

    /// 向 Matchers 添加 Matcher<NoticeEvent>
    pub fn add_notice_matcher(&mut self, matcher: Matcher<NoticeEvent>) -> &mut Self {
        Matchers::add_matcher(&mut self.notice, matcher, self.action_sender.clone());
        self
    }

    /// 向 Matchers 添加 Matcher<RequestEvent>
    pub fn add_request_matcher(&mut self, matcher: Matcher<RequestEvent>) -> &mut Self {
        Matchers::add_matcher(&mut self.request, matcher, self.action_sender.clone());
        self
    }

    /// 向 Matchers 添加 Matcher<MetaEvent>
    pub fn add_meta_matcher(&mut self, matcher: Matcher<MetaEvent>) -> &mut Self {
        Matchers::add_matcher(&mut self.meta, matcher, self.action_sender.clone());
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
