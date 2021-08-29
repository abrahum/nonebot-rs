use crate::utils::{format_division, get_data, R6sClient};
use nonebot_rs::{
    async_trait,
    event::MessageEvent,
    matcher::{Handler, Matcher},
    on_command,
};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct R6sPlays {
    pub client: Arc<R6sClient>,
}

#[async_trait]
impl Handler<MessageEvent> for R6sPlays {
    on_command!(MessageEvent, "R6sp", "R6p", "r6p", "r6sp");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let id = event.get_raw_message();
        match get_data(&(*self.client), id).await {
            Ok(data) => {
                let text = format_plays(id, data.get("StatCR2").unwrap());
                matcher.send_text(&text).await;
            }
            Err(e) => matcher.send_text(e).await,
        }
    }
}

fn format_plays(id: &str, data: &Value) -> String {
    let f = |data: &Value| {
        let update_at = data.get("update_at").unwrap();
        let date = format!(
            "{}.{}.{} {}:{}",
            update_at.get("year").unwrap().as_i64().unwrap() + 1900,
            update_at.get("month").unwrap().as_i64().unwrap() + 1,
            update_at.get("date").unwrap().as_i64().unwrap(),
            update_at.get("hours").unwrap(),
            update_at.get("minutes").unwrap()
        );
        let kd = if data.get("deaths").unwrap() == 0 {
            "-".to_string()
        } else {
            format_division("kills", "deaths", &data)
        };
        format!(
            "{}\n胜负比：{}/{}\nKD：{}",
            date,
            data.get("won").unwrap(),
            data.get("lost").unwrap(),
            kd,
        )
    };

    let mut s = format!("{} 近期对战：\n", id);
    for i in 0..3 {
        s = format!("{}\n{}", s, f(data.get(i).unwrap()))
    }
    s
}
