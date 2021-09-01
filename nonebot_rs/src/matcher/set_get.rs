use super::{Handler, Matcher, PreMatcher, Rule};
use std::sync::Arc;

impl<E> Matcher<E>
where
    E: Clone,
{
    /// 设置 Matcher 的 api_sender 与 api_resp_watcher
    pub fn build(
        &self,
        api_sender: crate::ApiSender,
        api_resp_watcher: crate::ApiRespWatcher,
    ) -> Matcher<E> {
        let mut m = self.clone();
        if let None = &m.api_sender {
            m.api_sender = Some(api_sender);
        }
        m.api_resp_watcher = Some(api_resp_watcher);
        m
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
