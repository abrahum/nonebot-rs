use reqwest::{header::HeaderMap, Client};
use serde_json::Value;

#[derive(Clone)]
pub struct R6sClient {
    pub client: Client,
    pub headers: HeaderMap,
}

pub async fn get_data<'a>(client: &R6sClient, id: &'a str) -> Result<Value, &'a str> {
    if let Ok(resp) = client
        .client
        .get(format!(
            "https://www.r6s.cn/Stats?username={}&platform=",
            id
        ))
        .headers(client.headers.clone())
        .send()
        .await
    {
        match resp.json::<Value>().await {
            Ok(value) => Ok(value),
            Err(_) => Err("数据格式有误。"),
        }
    } else {
        Err("R6cn 又抽风啦。")
    }
}

pub fn format_division(word1: &str, word2: &str, data: &Value) -> String {
    let w1 = data.get(word1).unwrap().as_i64().unwrap();
    let w2 = data.get(word2).unwrap().as_i64().unwrap();
    if w2 != 0 {
        format!("{:.2}", w1 as f64 / w2 as f64)
    } else {
        format!("{}/{}", w1, w2)
    }
}

pub fn format_stat(data: &Value) -> String {
    let kd = format_division("kills", "deaths", data);
    let wl = format_division("won", "lost", data);
    let timeplayed: f64 = data.get("timePlayed").unwrap().as_f64().unwrap() as f64 / 3600.0;
    format!(
        "KD：{}\n胜负比：{}\n总场数：{}\n游戏时常：{:.2}",
        kd,
        wl,
        data.get("played").unwrap(),
        timeplayed
    )
}
