use crate::utils::{format_stat, get, get_data, R6sClient};
use nonebot_rs::{
    async_trait,
    event::MessageEvent,
    matcher::{Handler, Matcher},
    on_command,
};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct R6s {
    pub client: Arc<R6sClient>,
}

#[async_trait]
impl Handler<MessageEvent> for R6s {
    on_command!(MessageEvent, "R6s", "r6s", "R6", "r6");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let nickname = get(event);
        if let Some(nickname) = nickname {
            match get_data(&(*self.client), &nickname).await {
                Ok(data) => {
                    if data == Value::Object(serde_json::map::Map::new()) {
                        matcher.send_text("干员数据为空").await;
                        return;
                    }
                    let text = format_base(&nickname, data);
                    matcher.send_text(&text).await;
                }
                Err(e) => matcher.send_text(e).await,
            }
        } else {
            matcher.send_text("请先使用r6sset设置昵称后查询").await;
            return;
        }
    }
}

fn format_base(id: &str, data: Value) -> String {
    format!(
        "{}\n等级：{}\n\n综合数据：\n{}",
        id,
        data.get("Basicstat")
            .unwrap()
            .get(0)
            .unwrap()
            .get("level")
            .unwrap(),
        format_stat(data.get("StatGeneral").unwrap().get(0).unwrap())
    )
}
