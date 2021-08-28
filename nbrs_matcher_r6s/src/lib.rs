use nonebot_rs::builtin::prematcher::*;
use nonebot_rs::event::MessageEvent;
use nonebot_rs::matcher::{Handler, Matcher};
use nonebot_rs::{async_trait, on_command};
use reqwest::{header::HeaderMap, Client};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct R6s {
    client: Client,
    headers: HeaderMap,
}

#[async_trait]
impl Handler<MessageEvent> for R6s {
    on_command!(MessageEvent, "R6s", "R6", "r6", "r6s");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let id = event.get_raw_message();
        if let Ok(resp) = self
            .client
            .get(format!(
                "https://www.r6s.cn/Stats?username={}&platform=",
                id
            ))
            .headers(self.headers.clone())
            .send()
            .await
        {
            let data = resp.json::<Value>().await;
            if let Ok(data) = data {
                let text = format_data(id, data);
                matcher.send_text(&text).await;
            } else {
                matcher.send_text("format data fail").await;
            }
        } else {
            matcher.send_text(&format!("{} not found", id)).await;
        }
    }
}

fn format_data(id: &str, data: Value) -> String {
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

fn format_stat(data: &Value) -> String {
    fn f(word1: &str, word2: &str, data: &Value) -> String {
        let w1 = data.get(word1).unwrap().as_i64().unwrap();
        let w2 = data.get(word2).unwrap().as_i64().unwrap();
        if w2 != 0 {
            format!("{:.2}", w1 as f64 / w2 as f64)
        } else {
            format!("{}/{}", w1, w2)
        }
    }
    let kd = f("kills", "deaths", data);
    let wl = f("won", "lost", data);
    let timeplayed: f64 = data.get("timePlayed").unwrap().as_f64().unwrap() as f64 / 3600.0;
    format!(
        "KD：{}\n胜负比：{}\n总场数：{}\n游戏时常：{:.2}",
        kd,
        wl,
        data.get("played").unwrap(),
        timeplayed
    )
}

pub fn r6s() -> Matcher<MessageEvent> {
    let mut headers = HeaderMap::new();
    headers.insert("Host", "www.r6s.cn".parse().unwrap());
    headers.insert("referer", "https://www.r6s.cn".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
    Matcher::new(
        "r6s".to_string(),
        R6s {
            client: Client::new(),
            headers: headers,
        },
    )
    .add_pre_matcher(to_me())
    .add_pre_matcher(Arc::new(command_start))
}
