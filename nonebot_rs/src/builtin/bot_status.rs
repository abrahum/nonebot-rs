use crate::matcher::prelude::*;

struct Status {}

#[async_trait]
impl Handler<MessageEvent> for Status {
    crate::on_command!(MessageEvent, "status");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        matcher
            .send_text(&build_status(&event, &matcher).await)
            .await;
    }
}

async fn build_status(event: &MessageEvent, matcher: &Matcher<MessageEvent>) -> String {
    let friend_count = match matcher.get_friend_list().await {
        Some(flist) => flist.len(),
        None => 0,
    };
    let group_count = match matcher.get_group_list().await {
        Some(glist) => glist.len(),
        None => 0,
    };
    format!(
        "当前BotId：{}\n已加载好友数量：{}\n已加载群数量：{}",
        event.get_self_id(),
        friend_count,
        group_count
    )
}

pub fn bot_status() -> Matcher<MessageEvent> {
    Matcher::new("Bot Status", Status {}).add_rule(rules::is_superuser())
}
