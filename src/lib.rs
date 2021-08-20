mod axum;
mod bot;
mod config;
mod event;
mod log;
mod matcher;
mod message;

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

pub struct Nonebot {
    // 作为根结构体，用来传递各种全局变量和设定值
    pub bots: HashMap<String, bot::Bot>, // 每个 websocket 链接构建一个 Bot 对象并存储
    pub matchers: BTreeMap<u8, Vec<matcher::Matcher>>, // 按照优先级存储 Matcher
    pub config: config::NbConfig,        // 全局设置
}

impl Default for Nonebot {
    fn default() -> Self {
        Nonebot {
            bots: HashMap::new(),
            matchers: BTreeMap::new(),
            config: config::NbConfig::load(),
        }
    }
}

impl Nonebot {
    #[tokio::main]
    pub async fn run(mut self) {
        use crate::log;
        log::init();

        use crate::axum;
        axum::run(Arc::new(Mutex::new(self))).await;
    }
}
