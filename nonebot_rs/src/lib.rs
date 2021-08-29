#![doc(html_favicon_url = "https://v2.nonebot.dev/logo.png")]
#![doc(html_logo_url = "https://v2.nonebot.dev/logo.png")]

//! Nonebot-rs 简称 nbrs，是一个基于 Nonebot2 思路的 Onebot 协议机器人框架 Rust 实现。
//! 本框架的基本目标是实现比较便利的 Rust Onebot 机器人搭建。长期目标是以本项目为基础，
//! 开发与其他脚本语言（比如：Python、Lua）互通的 Onebot 机器人平台（如果我能坚持下去
//! 的话）。
//!
//! 基于本框架实现的机器人，可以由一下几部分组成：nbrs 核心、Matcher 插件、启动文件，
//! 每个部分均可独立为单个 crate ，通过启动文件向 nbrs 注册 Matcher 后编译启动的方式
//! 构建一个机器人实例。
//!
//! # nbrs 设计
//!
//! nbrs 启动后，将读取设置文件、并注册 Matchers（其实这一步已经在编译时硬编码），当接
//! 收到 WebSocket 连接后，将新建一个 Bot 实例，接受 Event 后，由 Bot 负责逐渐匹配分发
//! 到各个 Matcher ，Matcher 处理后，通过 channel 将数据传递回 WebSocket 发送。每个
//! Event 的匹配与 Matcher 的处理均为独立协程，以此提高并发性能。

/////////////////////////////////////////////////////////////////////////////////

pub mod api;
pub mod axum_driver;
pub mod bot;
/// 内建组件
pub mod builtin;
pub mod config;
pub mod event;
mod log;
/// Matcher 定义
pub mod matcher;
/// Onebot 消息接口定义
pub mod message;
mod results;
mod utils;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[doc(inline)]
pub use api::Api;
pub use async_trait::async_trait;
#[doc(inline)]
pub use matcher::matchers::{Matchers, MatchersBTreeMap, MatchersHashMap};
#[doc(inline)]
pub use message::Message;

#[macro_use]
extern crate lazy_static;

/// Bot 状态暂存
///
/// `Nonebot.bots` 结构体中保存的对象，请注意与 `nonebot_rs::bot::Bot` 的区分，本结构体
/// 用于储存从配置中读取的 `BotConfig`、后续跨 `Bot` 通讯也需要从该结构体查询。
#[derive(Debug)]
pub struct Bot {
    pub superusers: Vec<String>,
    pub nickname: Vec<String>,
    pub command_start: Vec<String>,
    pub sender: Option<bot::ApiSender>,
}

/// nbrs 本体
///
/// 用于注册 `Matcher`，暂存配置项，以及启动实例
pub struct Nonebot {
    pub config: config::NbConfig, // 全局设置
    pub bots: HashMap<String, Bot>,
    pub matchers: Matchers,
}

impl Nonebot {
    /// 根据输入的 `NbConfig` 生成 `Nonebot.bots` 储存的配置信息
    fn build_bots(config: &config::NbConfig) -> HashMap<String, Bot> {
        let mut rmap = HashMap::new();
        if let Some(bots_config) = &config.bots {
            for (bot_id, bot_config) in bots_config {
                rmap.insert(
                    bot_id.clone(),
                    Bot {
                        sender: None,
                        superusers: bot_config.superusers.clone(),
                        nickname: bot_config.nickname.clone(),
                        command_start: bot_config.command_start.clone(),
                    },
                );
            }
        }
        rmap
    }

    /// 当 WenSocket 收到配置中未配置的 Bot 时，调用该方法新建 Bot 配置信息
    pub fn add_bot(&mut self, bot_id: i64, sender: bot::ApiSender) {
        let bot_id = bot_id.to_string();
        if let Some(bot) = self.bots.get_mut(&bot_id) {
            bot.sender = Some(sender);
        } else {
            self.bots.insert(
                bot_id,
                Bot {
                    sender: Some(sender),
                    superusers: self.config.global.superusers.clone(),
                    nickname: self.config.global.nickname.clone(),
                    command_start: self.config.global.command_start.clone(),
                },
            );
        }
    }

    /// 新建一个 Matchers 为空的 Nonebot 结构体
    pub fn new() -> Self {
        let config = config::NbConfig::load();
        Nonebot {
            bots: Nonebot::build_bots(&config),
            config: config::NbConfig::load(),
            matchers: Matchers::new(None, None, None, None),
        }
    }

    pub fn pre_run(&self) {
        use colored::*;
        log::init(self.config.global.debug, self.config.global.trace);
        tracing::event!(
            tracing::Level::INFO,
            "{}",
            "高性能自律実験4号機が稼働中····".red()
        );
    }

    /// 运行 Nonebot 实例
    pub fn run(self) {
        self.pre_run();
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(axum_driver::run(Arc::new(Mutex::new(self))));
    }
}
