use crate::bot::ApiSender;
use crate::event::{Events, MessageEvent};
use crate::results::AfterMatcherResult;
use crate::Nonebot;
use async_trait::async_trait;
use colored::*;
use std::sync::{Arc, Mutex};
use tracing::{event as tevent, Level};

pub type AMNb = Arc<Mutex<Nonebot>>;
pub type Rule<E> = fn(&E, AMNb) -> bool;
pub type PreMatcher<E> = fn(&E, AMNb) -> Option<E>;
pub type AfterMatcher = fn(&Events, AMNb) -> AfterMatcherResult;

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
    after_matchers: Vec<Arc<AfterMatcher>>,     // todo 还没想好干啥的后处理器
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
    fn match_(&self, event: &mut E) -> bool; // 匹配命令，类比 nb2 的 on 装饰器
    async fn handle(&self, event: E, matcher: Matcher<E>); // 事件处理函数
}

impl<E> Matcher<E>
where
    E: Clone,
{
    pub fn new(name: String, handler: Arc<dyn Handler<E> + Sync + Send>) -> Matcher<E> {
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

    pub fn set_sender(&mut self, sender: ApiSender) -> Matcher<E> {
        self.sender = Some(sender);
        self.clone()
    }

    pub fn add_pre_matcher(&mut self, pre_matcher: Arc<PreMatcher<E>>) -> Matcher<E> {
        self.pre_matchers.push(pre_matcher);
        self.clone()
    }

    pub fn add_after_matcher(&mut self, after_matcher: Arc<AfterMatcher>) -> Matcher<E> {
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

    pub fn prematcher_handle(&self, event: E, amnb: AMNb) -> Option<E> {
        let mut revent = event.clone();
        for premather in &self.pre_matchers {
            match premather(&revent, amnb.clone()) {
                Some(e) => revent = e,
                None => return None,
            }
        }
        Some(revent)
    }

    fn check_rules(&self, event: &E, nb: AMNb) -> bool {
        // 一次性检查当前事件是否满足所有 Rule
        // check the event fit all the rules or not
        for rule in &self.rules {
            if !rule(event, nb.clone()) {
                return false;
            }
        }
        true
    }

    pub async fn match_(&self, event: E, nb: AMNb) -> bool
    where
        E: Send + 'static,
    {
        if let Some(mut e) = self.prematcher_handle(event, nb.clone()) {
            if !self.check_rules(&e, nb.clone()) {
                return false;
            }
            let handler = self.handler.clone();
            if !handler.match_(&mut e) {
                return false;
            }
            let matcher = self.clone().set_event(&e);
            tokio::spawn(async move { handler.handle(e, matcher).await });
            return true;
        } else {
            false
        }
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
