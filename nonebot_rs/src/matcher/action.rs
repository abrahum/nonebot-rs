use crate::log::{colored::*, event, Level};

/// Matchers 内部 Action
#[derive(Clone, Debug)]
pub enum MatchersAction {
    /// 添加 MessageEvent Matcher
    AddMessageEventMatcher {
        message_event_matcher: super::Matcher<crate::event::MessageEvent>,
    },
    /// 移除 Matcher
    RemoveMatcher { matcher_name: String },
}

impl super::matchers::Matchers {
    /// Matchers 处理 action method
    pub fn handle_action(&mut self, action: MatchersAction) {
        match action {
            MatchersAction::AddMessageEventMatcher {
                message_event_matcher,
            } => {
                event!(
                    Level::DEBUG,
                    "Adding Message Event Matcher: {}",
                    message_event_matcher.name.blue()
                );
                self.add_message_matcher(message_event_matcher);
            }
            MatchersAction::RemoveMatcher { matcher_name } => {
                event!(
                    Level::DEBUG,
                    "Removing Message Event Matcher: {}",
                    matcher_name.blue()
                );
                self.remove_matcher(&matcher_name);
            }
        }
    }
}
