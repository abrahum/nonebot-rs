use crate::bot::{ApiRespWatcher, ApiSender, ChannelItem};
use crate::config::BotConfig;
use crate::event::{MessageEvent, SelfId};
use crate::utils::timestamp;
use async_trait::async_trait;
use colored::*;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{event as tevent, Level};

pub mod matchers;

pub type Rule<E> = Arc<dyn Fn(&E, &BotConfig) -> bool + Send + Sync>;
pub type PreMatcher<E> = fn(&mut E, BotConfig) -> bool;
// pub type AfterMatcher<E> = fn(&mut E, BotConfig) -> AfterMatcherResult;

/// 参与匹配的最小单元
///
/// Matcher 匹配器，每个匹配器对应一个 handle 函数
#[derive(Clone)]
pub struct Matcher<E>
where
    E: Clone,
{
    /// Matcher 名称，是 Matcher 的唯一性标识
    pub name: String,
    /// 消息发送接口
    sender: Option<ApiSender>,
    /// Api 回执接受通道
    watcher: Option<ApiRespWatcher>,
    /// Matcher 的匹配优先级
    pub priority: i8,
    /// 前处理函数组，获取 &mut event
    pre_matchers: Vec<Arc<PreMatcher<E>>>,
    /// rule 组
    rules: Vec<Rule<E>>,
    /// 是否阻止事件向下一级传递
    block: bool,
    /// Matcher 接口函数与可配置项结构体
    handler: Arc<dyn Handler<E> + Sync + Send>,
    /// 是否被禁用
    disable: bool,
    /// 是否为临时 Matcher
    temp: bool,
    /// 过期时间戳
    pub timeout: Option<i64>,

    #[doc(hidden)]
    event: Option<E>,
}

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
    /// 匹配函数
    fn match_(&self, event: &mut E) -> bool;
    /// 处理函数
    async fn handle(&self, event: E, matcher: Matcher<E>);
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
    ///     sender: None,
    ///     priority: 1,
    ///     pre_matchers: vec![],
    ///     rules: vec![],
    ///     block: true,
    ///     handler: Arc::new(handler),
    ///     disable: false,
    ///     temp: false,
    ///     timeout: None,
    ///     event: None,
    /// }
    /// ```
    pub fn new<H>(name: String, handler: H) -> Matcher<E>
    where
        H: Handler<E> + Sync + Send + 'static,
    {
        // 默认 Matcher
        Matcher {
            name: name,
            sender: None,
            watcher: None,
            priority: 1,
            pre_matchers: vec![],
            // after_matchers: vec![],
            rules: vec![],
            block: true,
            handler: Arc::new(handler),
            disable: false,
            temp: false,
            timeout: None,

            event: None,
        }
    }

    fn pre_matcher_handle(&self, event: &mut E, config: BotConfig) -> bool {
        // 遍历 pre_matcher 处理
        for premather in &self.pre_matchers {
            if !premather(event, config.clone()) {
                return false;
            }
        }
        true
    }

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

    pub async fn match_(&self, event: E, config: BotConfig) -> bool
    where
        E: Send + 'static + SelfId,
    {
        // Matcher 处理流程，匹配成功返回 true 并行处理 handler
        let mut event = event.clone();
        if let Some(timeout) = self.timeout {
            if timestamp() > timeout {
                self.set(crate::bot::Action::RemoveMatcher {
                    bot_id: event.get_self_id(),
                    name: self.name.clone(),
                })
                .await;
                return false;
            }
        }
        if !self.pre_matcher_handle(&mut event, config.clone()) {
            return false;
        }
        if !self.check_rules(&event, &config) {
            return false;
        }
        let handler = self.handler.clone();
        if !handler.match_(&mut event) {
            return false;
        }
        let matcher = self.clone().set_event(&event);
        tokio::spawn(async move { handler.handle(event, matcher).await });
        return true;
    }

    pub async fn call_api(&self, api: crate::api::Api) {
        self.sender
            .clone()
            .unwrap()
            .send(ChannelItem::Api(api))
            .await
            .unwrap();
    }

    pub async fn set(&self, set: crate::bot::Action) {
        self.sender
            .clone()
            .unwrap()
            .send(ChannelItem::Action(set))
            .await
            .unwrap();
    }

    pub async fn set_message_matcher(&self, bot_id: String, matcher: Matcher<MessageEvent>) {
        let set = crate::bot::Action::AddMessageEventMatcher {
            bot_id: bot_id,
            message_event_matcher: matcher,
        };
        self.set(set).await;
    }

    pub async fn set_temp_message_event_matcher<H>(&self, event: &MessageEvent, handler: H)
    where
        H: Handler<MessageEvent> + Send + Sync + 'static,
    {
        self.set_message_matcher(
            event.get_self_id(),
            build_temp_message_event_matcher(event, handler),
        )
        .await;
    }
}

pub fn build_temp_message_event_matcher<H>(
    event: &MessageEvent,
    handler: H,
) -> Matcher<MessageEvent>
where
    H: Handler<MessageEvent> + Send + Sync + 'static,
{
    use crate::event::UserId;
    let mut m = Matcher::new(
        format!("{}-{}", event.get_user_id(), event.get_time()),
        handler,
    )
    .add_rule(crate::builtin::rules::is_user(event.get_user_id()));
    if let MessageEvent::Group(g) = event {
        m.add_rule(crate::builtin::rules::in_group(g.group_id));
    } else {
        m.add_rule(crate::builtin::rules::is_private_message_event());
    }
    m.set_priority(0).set_temp(true)
}

impl<E> Matcher<E>
where
    E: Clone,
{
    pub fn set_sender(&mut self, sender: ApiSender) -> Matcher<E> {
        self.sender = Some(sender);
        self.clone()
    }

    pub fn get_sender(&mut self) -> Option<ApiSender> {
        self.sender.clone()
    }

    pub fn set_watcher(&mut self, watcher: ApiRespWatcher) -> Matcher<E> {
        self.watcher = Some(watcher);
        self.clone()
    }

    pub fn set_priority(&mut self, priority: i8) -> Matcher<E> {
        self.priority = priority;
        self.clone()
    }

    pub fn add_pre_matcher(&mut self, pre_matcher: Arc<PreMatcher<E>>) -> Matcher<E> {
        self.pre_matchers.push(pre_matcher);
        self.clone()
    }

    pub fn add_rule(&mut self, rule: Rule<E>) -> Matcher<E> {
        self.rules.push(rule);
        self.clone()
    }

    pub fn set_block(&mut self, block: bool) -> Matcher<E> {
        self.block = block;
        self.clone()
    }

    pub fn get_handler(&self) -> &Arc<dyn Handler<E> + Sync + Send> {
        &self.handler
    }

    pub fn set_handler(&mut self, handler: Arc<dyn Handler<E> + Sync + Send>) -> Matcher<E> {
        self.handler = handler;
        self.clone()
    }

    pub fn set_disable(&mut self, disable: bool) -> Matcher<E> {
        self.disable = disable;
        self.clone()
    }

    fn set_event(&mut self, event: &E) -> Matcher<E> {
        self.event = Some(event.clone());
        self.clone()
    }

    pub fn is_block(&self) -> bool {
        self.block
    }

    pub fn is_temp(&self) -> bool {
        self.temp
    }

    pub fn set_temp(&mut self, temp: bool) -> Matcher<E> {
        self.temp = temp;
        self.clone()
    }

    pub fn set_timeout(&mut self, timeout: i64) -> Matcher<E> {
        self.timeout = Some(timeout);
        self.clone()
    }
}

impl<E> Matcher<E>
where
    E: Clone,
{
    pub async fn call_api_resp(&self, api: crate::Api) -> Option<crate::api::ApiResp> {
        let echo = api.get_echo();
        self.sender
            .clone()
            .unwrap()
            .send(crate::bot::ChannelItem::Api(api))
            .await
            .unwrap();
        let mut watcher = self.watcher.clone().unwrap();
        let time = crate::utils::timestamp();
        while let Ok(_) = watcher.changed().await {
            let resp = (*watcher.borrow()).clone();
            if resp.echo == echo {
                return Some(resp.clone());
            }
            if crate::utils::timestamp() > time + 30 {
                return None;
            }
        }
        None
    }
}

impl Matcher<MessageEvent> {
    pub async fn send_text(&self, msg: &str) {
        let msg = crate::message::Message::Text {
            text: msg.to_string(),
        };
        self.send(vec![msg]).await;
    }

    pub async fn request_message(&self, msg: &str) -> String {
        struct Temp {}

        #[async_trait]
        impl Handler<MessageEvent> for Temp {
            crate::on_match_all!();
            async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
                matcher
                    .sender
                    .clone()
                    .unwrap()
                    .send(crate::bot::ChannelItem::MessageEvent(event))
                    .await
                    .unwrap();
            }
        }

        let (sender, mut receiver) = mpsc::channel::<crate::bot::ChannelItem>(4);
        let event = self.event.clone().unwrap();
        self.set_message_matcher(
            event.get_self_id(),
            build_temp_message_event_matcher(&event, Temp {}).set_sender(sender),
        )
        .await;

        self.send_text(msg).await;
        while let Some(data) = receiver.recv().await {
            match data {
                crate::bot::ChannelItem::MessageEvent(event) => {
                    return event.get_raw_message().to_string()
                }
                _ => {
                    use colored::*;
                    tracing::event!(
                        tracing::Level::WARN,
                        "{}",
                        "Temp Matcher接受端接收到错误Api或Action消息".bright_red()
                    );
                } // 忽视 event 该 receiver 永不应该收到 event
            }
        }

        "".to_string()
    }

    pub async fn send(&self, msg: Vec<crate::message::Message>) {
        match self.event.clone().unwrap() {
            MessageEvent::Private(p) => {
                let info = format!("Send {:?} to {}({})", msg, p.sender.nickname, p.user_id,);
                tevent!(
                    Level::INFO,
                    "Send {:?} to {}({})",
                    msg,
                    p.sender.nickname.blue(),
                    p.user_id.to_string().green(),
                );
                &self
                    .sender
                    .clone()
                    .unwrap()
                    .send(ChannelItem::Api(crate::api::Api::SendPrivateMsg {
                        params: crate::api::SendPrivateMsg {
                            user_id: p.user_id,
                            message: msg,
                            auto_escape: false,
                        },
                        echo: info,
                    }))
                    .await
                    .unwrap();
            }
            MessageEvent::Group(g) => {
                let info = format!("Send {:?} to group ({})", msg, g.group_id,);
                tevent!(
                    Level::INFO,
                    "Send {:?} to group ({})",
                    msg,
                    g.group_id.to_string().magenta(),
                );
                self.sender
                    .clone()
                    .unwrap()
                    .send(ChannelItem::Api(crate::api::Api::SendGroupMsg {
                        params: crate::api::SendGroupMsg {
                            group_id: g.group_id,
                            message: msg,
                            auto_escape: false,
                        },
                        echo: info,
                    }))
                    .await
                    .unwrap();
            }
        }
    }
}
