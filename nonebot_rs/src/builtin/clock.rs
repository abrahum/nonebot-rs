use crate::async_trait;
use crate::event::MessageEvent;
use crate::matcher::Handler;
use crate::matcher::Matcher;
use crate::on_match_all;

pub struct Clock {
    crom: String,
}
#[async_trait]
impl Handler<MessageEvent> for Clock {
    on_match_all!();
    async fn handle(&self, _: MessageEvent, _: Matcher<MessageEvent>) {}
}
