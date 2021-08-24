use crate::bot::ApiSender;
use crate::event::Events;
use crate::results::{AfterMatcherResult, HandlerResult};
use crate::Nonebot;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

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
    pub pre_matchers: Vec<Arc<PreMatcher<E>>>, // 可以改变 event 的前处理器
    pub after_matchers: Vec<Arc<AfterMatcher>>, // todo 还没想好干啥的后处理器
    pub rules: Vec<Arc<Rule<E>>>,            // 所有需要被满足的 rule
    pub block: bool,                         // 是否阻止事件向下一级传递
    pub handler: Arc<dyn Handler<E> + Sync + Send>, // struct impl Handler trait
    pub disable: bool,                       // 禁用当前 Matcher
    pub ignore_command_start: bool,          // todo
}

#[async_trait]
pub trait Handler<E> {
    async fn handle(&self, event: E, sender: ApiSender) -> HandlerResult;
}

impl<E> Matcher<E>
where
    E: Clone,
{
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

    pub async fn match_(&self, event: E, nb: AMNb, sender: ApiSender) -> HandlerResult
    where
        E: Send + 'static,
    {
        if let Some(e) = self.prematcher_handle(event, nb.clone()) {
            if !self.check_rules(&e, nb.clone()) {
                return Ok(false);
            }
            let handler = self.handler.clone();
            let r = tokio::spawn(async move { handler.handle(e, sender).await });
            r.await.unwrap()
        } else {
            Ok(false)
        }
    }
}
