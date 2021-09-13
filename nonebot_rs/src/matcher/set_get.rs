use super::{Handler, Matcher, PreMatcher, Rule};
use std::sync::Arc;

impl<E> Matcher<E>
where
    E: Clone,
{
    /// 设置 Matcher 的 bot
    ///
    /// 当前 Matcher 如果已经预设 Bot 将会忽视传入的 Bot
    pub fn build(&self, bot: crate::bot::Bot) -> Matcher<E> {
        let mut m = self.clone();
        if let None = &m.bot {
            m.bot = Some(bot);
        }
        m
    }

    /// 为 Matcher 添加向 Matchers 发送 Matchers Action 的 Sender
    /// 会在向 Matchers 添加时调用
    pub fn set_action_sender(&mut self, action_sender: super::matchers::ActionSender) {
        self.action_sender = Some(action_sender);
    }

    /// 设置 priority
    pub fn set_priority(&mut self, priority: i8) -> Matcher<E> {
        self.priority = priority;
        self.clone()
    }

    /// 添加 pre_matcher 函数
    pub fn add_pre_matcher(&mut self, pre_matcher: Arc<PreMatcher<E>>) -> Matcher<E> {
        self.pre_matchers.push(pre_matcher);
        self.clone()
    }

    /// 添加 rule 函数
    pub fn add_rule(&mut self, rule: Rule<E>) -> Matcher<E> {
        self.rules.push(rule);
        self.clone()
    }

    /// 设置是否阻塞消息向下一级 priority 传递
    pub fn set_block(&mut self, block: bool) -> Matcher<E> {
        self.block = block;
        self.clone()
    }

    /// 获取 handler
    pub fn get_handler(&self) -> &Arc<dyn Handler<E> + Sync + Send> {
        &self.handler
    }

    /// 设置 handler
    pub fn set_handler(&mut self, handler: Arc<dyn Handler<E> + Sync + Send>) -> Matcher<E> {
        self.handler = handler;
        self.clone()
    }

    /// 设置是否 disable
    pub fn set_disable(&mut self, disable: bool) -> Matcher<E> {
        self.disable = disable;
        self.clone()
    }

    #[doc(hidden)]
    pub fn set_event(&mut self, event: &E) -> Matcher<E> {
        self.event = Some(event.clone());
        self.clone()
    }

    /// 返回 bolck
    pub fn is_block(&self) -> bool {
        self.block
    }

    /// 判定是否为临时 Matcher
    pub fn is_temp(&self) -> bool {
        self.temp
    }

    /// 设置是否为临时 Matcher
    pub fn set_temp(&mut self, temp: bool) -> Matcher<E> {
        self.temp = temp;
        self.clone()
    }

    /// 设置 Matcher 超时时限
    pub fn set_timeout(&mut self, timeout: i64) -> Matcher<E> {
        self.timeout = Some(timeout);
        self.clone()
    }
}
