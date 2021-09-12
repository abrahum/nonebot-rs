#[derive(Clone, Debug)]
pub enum MatchersAction {
    AddMessageEventMatcher {
        message_event_matcher: super::Matcher<crate::event::MessageEvent>,
    },
    RemoveMatcher {
        matcher_name: String,
    },
}

impl super::matchers::Matchers {
    pub fn handle_action(&mut self, action: MatchersAction) {
        match action {
            MatchersAction::AddMessageEventMatcher {
                message_event_matcher,
            } => {
                self.add_message_matcher(message_event_matcher);
            }
            MatchersAction::RemoveMatcher { matcher_name } => self.remove_matcher(&matcher_name),
        }
    }
}
