use crate::utils::{format_division, get, get_data, R6sClient};
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
    on_command!(MessageEvent, "R6sp", "r6sp", "R6p", "r6p");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let nickname = get(event);
        if let Some(nickname) = nickname {
            match get_data(&(*self.client), &nickname).await {
                Ok(data) => {
                    if data == Value::Object(serde_json::map::Map::new()) {
                        matcher.send_text("干员数据为空").await;
                        return;
                    }
                    let text = format_plays(&nickname, data.get("StatCR2").unwrap());
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
            "\n\n{}\n胜负比：{}/{}\nKD：{}",
            date,
            data.get("won").unwrap(),
            data.get("lost").unwrap(),
            kd,
        )
    };

    let mut s = format!("{} 近期对战：", id);
    for i in 0..3 {
        s = format!("{}{}", s, f(data.get(i).unwrap()))
    }
    s
}
