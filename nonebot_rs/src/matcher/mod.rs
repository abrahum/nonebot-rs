use crate::config::BotConfig;
use crate::event::{MessageEvent, SelfId};
use crate::utils::timestamp;
use crate::Action;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod action;
#[doc(hidden)]
pub mod api;
#[doc(hidden)]
pub mod matchers;
#[doc(hidden)]
pub mod message_event_matcher;
/// Preludo for Matcher
pub mod prelude;
#[doc(hidden)]
pub mod set_get;

/// rule 函数类型
pub type Rule<E> = Arc<dyn Fn(&E, &BotConfig) -> bool + Send + Sync>;
/// permatcher 函数类型
pub type PreMatcher<E> = fn(&mut E, BotConfig) -> bool;

/// 单个匹配器，参与匹配的最小单元
///
/// Matcher 匹配器，每个匹配器对应一个 handle 函数
#[derive(Clone)]
pub struct Matcher<E>
where
    E: Clone,
{
    /// Matcher 名称，是 Matcher 的唯一性标识
    pub name: String,
    /// Bot
    pub bot: Option<crate::bot::Bot>,
    /// Matchers Action Sender
    action_sender: Option<matchers::ActionSender>,
    /// Matcher 的匹配优先级
    pub priority: i8,
    /// 前处理函数组，获取 &mut event
    pre_matchers: Vec<Arc<PreMatcher<E>>>,
    /// rule 组
    rules: Vec<Rule<E>>,
    /// 是否阻止事件向下一级传递
    pub block: bool,
    /// Matcher 接口函数与可配置项结构体
    handler: Arc<RwLock<dyn Handler<E> + Sync + Send>>,
    /// 是否被禁用
    pub disable: bool,
    /// 是否为临时 Matcher
    pub temp: bool,
    /// 过期时间戳
    pub timeout: Option<i64>,

    #[doc(hidden)]
    event: Option<E>,
}

#[doc(hidden)]
impl<E> std::fmt::Debug for Matcher<E>
where
    E: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matcher")
            .field("name", &self.name)
            .field("priority", &self.priority)
            .field("block", &self.block)
            .field("disable", &self.disable)
            .field("temp", &self.temp)
            .field("timeout", &self.timeout)
            .field("bot", &self.bot)
            .finish()
    }
}

/// Matcher 接口 trait
#[async_trait]
pub trait Handler<E>
where
    E: Clone,
{
    /// 新 Bot 连接时，调用该函数
    fn on_bot_connect(&self, _: Matcher<E>) {}
    /// Bot 断开连接时，调用该函数
    fn on_bot_disconnect(&self, _: Matcher<E>) {}
    /// timeout drop 函数
    fn timeout_drop(&self, _: &Matcher<E>) {}
    /// 匹配函数
    fn match_(&self, event: &mut E) -> bool;
    /// 处理函数
    async fn handle(&self, event: E, matcher: Matcher<E>);
    /// Load config
    #[allow(unused_variables)]
    fn load_config(&mut self, config: HashMap<String, toml::Value>) {}
}

impl<E> Matcher<E>
where
    E: Clone,
{
    /// 生成默认模板 Matcher
    ///
    /// 默认模板：
    /// ``` rust
    /// Matcher {
    ///     name: name,
    ///     bot: None,
    ///     priority: 1,
    ///     pre_matchers: vec![],
    ///     rules: vec![],
    ///     block: true,
    ///     handler: Arc::new(RwLock::new(handler)),
    ///     disable: false,
    ///     temp: false,
    ///     timeout: None,
    ///     event: None,
    /// }
    /// ```
    pub fn new<H>(name: &str, handler: H) -> Matcher<E>
    where
        H: Handler<E> + Sync + Send + 'static,
    {
        // 默认 Matcher
        Matcher {
            name: name.to_string(),
            bot: None,
            action_sender: None,
            priority: 1,
            pre_matchers: vec![],
            rules: vec![],
            block: true,
            handler: Arc::new(RwLock::new(handler)),
            disable: false,
            temp: false,
            timeout: None,

            event: None,
        }
    }

    #[doc(hidden)]
    fn pre_matcher_handle(&self, event: &mut E, config: BotConfig) -> bool {
        // 遍历 pre_matcher 处理
        for premather in &self.pre_matchers {
            if !premather(event, config.clone()) {
                return false;
            }
        }
        true
    }

    #[doc(hidden)]
    fn check_rules(&self, event: &E, config: &BotConfig) -> bool {
        // 一次性检查当前事件是否满足所有 Rule
        // check the event fit all the rules or not
        for rule in &self.rules {
            if !rule(event, config) {
                return false;
            }
        }
        true
    }

    #[doc(hidden)]
    pub async fn match_(
        &self,
        event: E,
        config: BotConfig,
        matchers: &mut matchers::Matchers,
    ) -> bool
    where
        E: Send + 'static + SelfId,
    {
        // Matcher 处理流程，匹配成功返回 true 并行处理 handler
        let mut event = event.clone();
        if let Some(timeout) = self.timeout {
            if timestamp() > timeout {
                matchers.remove_matcher(&self.name);
                {
                    let handler = self.handler.read().await;
                    handler.timeout_drop(&self);
                }
                return false;
            }
        }
        if self.disable {
            return false;
        }
        if !self.pre_matcher_handle(&mut event, config.clone()) {
            return false;
        }
        if !self.check_rules(&event, &config) {
            return false;
        }
        {
            let handler = self.handler.read().await;
            if !handler.match_(&mut event) {
                return false;
            }
            let matcher = self.clone().set_event(&event);
            let handler = self.handler.clone();
            tokio::spawn(async move {
                let handler = handler.read().await;
                handler.handle(event, matcher).await
            });
        }
        return true;
    }

    /// 发送 nbrs 内部设置 Action
    pub async fn set(&self, set: Action) {
        if let Some(bot) = &self.bot {
            bot.action_sender.send(set).await.unwrap();
        }
    }

    /// 向 Matchers 添加 Matcher<MessageEvent>
    pub async fn set_message_matcher(&self, matcher: Matcher<MessageEvent>) {
        let action = action::MatchersAction::AddMessageEventMatcher {
            message_event_matcher: matcher,
        };
        if let Some(action_sender) = &self.action_sender {
            action_sender.send(action).unwrap();
        } else {
            tracing::event!(tracing::Level::WARN, "Action Sender not init.")
        }
    }
}

/// 构建 timeout 为 30s 的临时 Matcher<MessageEvent>
pub fn build_temp_message_event_matcher<H>(
    event: &MessageEvent,
    handler: H,
) -> Matcher<MessageEvent>
where
    H: Handler<MessageEvent> + Send + Sync + 'static,
{
    use crate::event::UserId;
    let mut m = Matcher::new(
        &format!(
            "{}-{}-{}",
            event.get_self_id(),
            event.get_user_id(),
            event.get_time()
        ),
        handler,
    )
    .add_rule(crate::builtin::rules::is_user(event.get_user_id()))
    .add_rule(crate::builtin::rules::is_bot(event.get_self_id()));
    if let MessageEvent::Group(g) = event {
        m.add_rule(crate::builtin::rules::in_group(g.group_id.clone()));
    } else {
        m.add_rule(crate::builtin::rules::is_private_message_event());
    }
    m.set_priority(0)
        .set_temp(true)
        .set_timeout(timestamp() + 30)
}
