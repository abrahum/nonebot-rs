use nonebot_rs::event::{MessageEvent, SelfId, UserId};
use reqwest::{header::HeaderMap, Client};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CACHE_DATE_PATH: &str = "cache";
const R6S_DIR_NAME: &str = "R6s";

pub type UserNicknameMap = HashMap<String, String>;

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

fn check_dir() -> PathBuf {
    let r6s_path = PathBuf::from(CACHE_DATE_PATH).join(R6S_DIR_NAME);
    if !r6s_path.exists() {
        fs::create_dir_all(&r6s_path).unwrap();
    }
    r6s_path
}

pub fn load(bot_id: &str) -> UserNicknameMap {
    let path = check_dir().join(format!("{}.json", bot_id));
    if path.exists() {
        let data = fs::read_to_string(path).unwrap();
        let data: UserNicknameMap = serde_json::from_str(&data).unwrap();
        data
    } else {
        let data = HashMap::new();
        data
    }
}

pub fn dump(bot_id: &str, data: UserNicknameMap) {
    let path = check_dir().join(format!("{}.json", bot_id));
    let data_str = serde_json::to_string(&data).unwrap();
    fs::write(path, &data_str).unwrap();
}

pub fn get(event: MessageEvent) -> Option<String> {
    let msg = event.get_raw_message();
    if !msg.is_empty() {
        return Some(msg.to_string());
    }
    let data = load(&event.get_self_id());
    data.get(&event.get_user_id()).and_then(|x| Some(x.clone()))
}

pub fn set(bot_id: &str, user_id: String, nickname: String) {
    let mut data = load(bot_id);
    data.insert(user_id, nickname);
    dump(bot_id, data);
}
