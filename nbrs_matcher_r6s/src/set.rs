use crate::utils::{set, R6sClient};
use nonebot_rs::{
    async_trait,
    event::{MessageEvent, SelfId, UserId},
    matcher::{Handler, Matcher},
    on_command,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct R6sSet {
    pub client: Arc<R6sClient>,
}

#[async_trait]
impl Handler<MessageEvent> for R6sSet {
    on_command!(MessageEvent, "R6sset", "r6sset", "R6set", "r6set");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let nickname = matcher
            .request_message(Some(&event), Some("请输入游戏昵称"))
            .await;
        if let Some(nickname) = nickname {
            set(
                &event.get_self_id(),
                event.get_user_id(),
                nickname.to_string(),
            );
            matcher
                .send_text(&format!("已设置昵称\n{}", nickname))
                .await;
        } else {
            matcher.send_text("无法设置该昵称").await;
        }
    }
}
