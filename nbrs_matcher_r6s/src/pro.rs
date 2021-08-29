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
pub struct R6sPro {
    pub client: Arc<R6sClient>,
}

#[async_trait]
impl Handler<MessageEvent> for R6sPro {
    on_command!(MessageEvent, "R6spro", "R6pro", "r6pro", "r6spro");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let id = event.get_raw_message();
        match get_data(&(*self.client), id).await {
            Ok(data) => {
                let text = format_pro(id, data);
                matcher.send_text(&text).await;
            }
            Err(e) => matcher.send_text(e).await,
        }
    }
}

fn format_pro(id: &str, data: Value) -> String {
    let casual_mmr = data.get("Casualstat").unwrap().get("mmr").unwrap();
    let casual = format! {
        "{}\n\n休闲数据：{}\n隐藏MMR：{}\n隐藏Rank：{}",
        id,
        format_stat(data.get("StatCR").unwrap().get(0).unwrap()),
        casual_mmr,
        rank(casual_mmr.as_i64().unwrap()),
    };
    if let Some(rank_data) = data.get("StatCR").unwrap().get(1) {
        let rank_mmr = data
            .get("Basicstat")
            .unwrap()
            .get(0)
            .unwrap()
            .get("mmr")
            .unwrap();
        return format!(
            "{}\n\n排位数据：{}\n排位MMR：{}\n排位Rank：{}",
            casual,
            format_stat(rank_data),
            rank_mmr,
            rank(rank_mmr.as_i64().unwrap()),
        );
    }
    casual
}

fn rank(mmr: i64) -> String {
    let f = |a: &str, b: &str| format!("{}{}", a, b);

    let head = ["紫铜", "黄铜", "白银", "黄金", "白金", "钻石", "冠军"];
    let feet1 = ["V", "IV", "III", "II", "I"];
    let feet2 = ["III", "II", "I"];
    if mmr < 2600 {
        let mmrd = (mmr / 100) - 11;
        if mmrd < 5 {
            return f(head[0], feet1[mmrd as usize]);
        } else if mmrd < 10 {
            return f(head[1], feet1[(mmrd - 5) as usize]);
        } else {
            return f(head[2], feet1[(mmrd - 10) as usize]);
        }
    } else if mmr < 4400 {
        let mmrd = (mmr / 200) - 13;
        if mmrd < 3 {
            return f(head[3], feet2[mmrd as usize]);
        } else {
            return f(head[4], feet2[((mmrd - 3) / 2) as usize]);
        }
    } else if mmr < 5000 {
        return head[5].to_string();
    } else {
        return head[6].to_string();
    }
}
