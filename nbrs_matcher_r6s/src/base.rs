use crate::utils::{format_stat, get_data, R6sClient};
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
    on_command!(MessageEvent, "R6s", "R6", "r6", "r6s");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let id = event.get_raw_message();
        match get_data(&(*self.client), id).await {
            Ok(data) => {
                let text = format_base(id, data);
                matcher.send_text(&text).await;
            }
            Err(e) => matcher.send_text(e).await,
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
