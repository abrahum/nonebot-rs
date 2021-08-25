use crate::bot::ApiSender;
use crate::config::BotConfig;
use crate::event::MessageEvent;
use crate::results::AfterMatcherResult;
use async_trait::async_trait;
use colored::*;
use std::sync::Arc;
use tracing::{event as tevent, Level};

pub type Rule<E> = fn(&E, &BotConfig) -> bool;
pub type PreMatcher<E> = fn(&mut E, BotConfig) -> bool;
pub type AfterMatcher<E> = fn(&mut E, BotConfig) -> AfterMatcherResult;

#[derive(Clone)]
pub struct Matcher<E>
where
    E: Clone,
{
    // Matcher 匹配器，每个匹配器对应一个 handle 函数
    pub name: String,                           // 名称（需要唯一性）
    sender: Option<ApiSender>,                  // 发送器
    pub priority: i8,                           // 匹配优先级
    pre_matchers: Vec<Arc<PreMatcher<E>>>,      // 可以改变 event 的前处理器
    after_matchers: Vec<Arc<AfterMatcher<E>>>,  // todo 注销 temp Matcher 等
    rules: Vec<Arc<Rule<E>>>,                   // 所有需要被满足的 rule
    block: bool,                                // 是否阻止事件向下一级传递
    handler: Arc<dyn Handler<E> + Sync + Send>, // struct impl Handler trait
    disable: bool,                              // 禁用当前 Matcher
    ignore_command_start: bool,                 // todo

    event: Option<E>,
}

#[async_trait]
pub trait Handler<E>
where
    E: Clone,
{
    // 所有 Matcher 需要实现的匹配与业务函数
    fn match_(&self, event: &mut E) -> bool; // 匹配命令，类比 nb2 的 on 装饰器
    async fn handle(&self, event: E, matcher: Matcher<E>); // 事件处理函数
}

impl<E> Matcher<E>
where
    E: Clone,
{
    pub fn new(name: String, handler: Arc<dyn Handler<E> + Sync + Send>) -> Matcher<E> {
        // 默认 Matcher
        Matcher {
            name: name,
            sender: None,
            priority: 1,
            pre_matchers: vec![],
            after_matchers: vec![],
            rules: vec![],
            block: true,
            handler: handler,
            disable: false,
            ignore_command_start: false,

            event: None,
        }
    }

    pub fn pre_matcher_handle(&self, event: &mut E, config: BotConfig) -> bool {
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
        E: Send + 'static,
    {
        // Matcher 处理流程，匹配成功返回 true 并行处理 handler
        let mut event = event.clone();
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
}

impl<E> Matcher<E>
where
    E: Clone,
{
    pub fn set_sender(&mut self, sender: ApiSender) -> Matcher<E> {
        self.sender = Some(sender);
        self.clone()
    }

    pub fn add_pre_matcher(&mut self, pre_matcher: Arc<PreMatcher<E>>) -> Matcher<E> {
        self.pre_matchers.push(pre_matcher);
        self.clone()
    }

    pub fn add_after_matcher(&mut self, after_matcher: Arc<AfterMatcher<E>>) -> Matcher<E> {
        self.after_matchers.push(after_matcher);
        self.clone()
    }

    pub fn add_rule(&mut self, rule: Arc<Rule<E>>) -> Matcher<E> {
        self.rules.push(rule);
        self.clone()
    }

    pub fn set_block(&mut self, block: bool) -> Matcher<E> {
        self.block = block;
        self.clone()
    }

    pub fn set_handler(&mut self, handler: Arc<dyn Handler<E> + Sync + Send>) -> Matcher<E> {
        self.handler = handler;
        self.clone()
    }

    pub fn set_disable(&mut self, disable: bool) -> Matcher<E> {
        self.disable = disable;
        self.clone()
    }

    pub fn set_ignore_command_start(&mut self, ignore_command_start: bool) -> Matcher<E> {
        self.ignore_command_start = ignore_command_start;
        self.clone()
    }

    fn set_event(&mut self, event: &E) -> Matcher<E> {
        self.event = Some(event.clone());
        self.clone()
    }

    pub fn is_block(&self) -> bool {
        self.block
    }
}

impl Matcher<MessageEvent> {
    pub async fn send(&self, msg: Vec<crate::message::Message>) {
        match self.event.clone().unwrap() {
            MessageEvent::Private(p) => {
                let info = format!("echo {:?} to {}({})", msg, p.sender.nickname, p.user_id,);
                tevent!(
                    Level::INFO,
                    "echo {:?} to {}({})",
                    msg,
                    p.sender.nickname.blue(),
                    p.user_id.to_string().green(),
                );
                &self
                    .sender
                    .clone()
                    .unwrap()
                    .send(crate::api::Apis::SendPrivateMsg {
                        params: crate::api::SendPrivateMsg {
                            user_id: p.user_id,
                            message: msg,
                            auto_escape: false,
                        },
                        echo: info,
                    })
                    .await
                    .unwrap();
            }
            MessageEvent::Group(g) => {
                let info = format!("echo {:?} to group ({})", msg, g.group_id,);
                tevent!(
                    Level::INFO,
                    "echo {:?} to group ({})",
                    msg,
                    g.group_id.to_string().magenta(),
                );
                self.sender
                    .clone()
                    .unwrap()
                    .send(crate::api::Apis::SendGroupMsg {
                        params: crate::api::SendGroupMsg {
                            group_id: g.group_id,
                            message: msg,
                            auto_escape: false,
                        },
                        echo: info,
                    })
                    .await
                    .unwrap();
            }
        }
    }
}
