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
    let friend_count = match matcher
        .call_api_resp(crate::Api::get_friend_list())
        .await
        .unwrap()
        .data
    {
        crate::RespData::FriendList(flist) => flist.len(),
        _ => 0,
    };
    let group_count = match matcher
        .call_api_resp(crate::Api::get_group_list())
        .await
        .unwrap()
        .data
    {
        crate::RespData::GroupList(glist) => glist.len(),
        _ => 0,
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
