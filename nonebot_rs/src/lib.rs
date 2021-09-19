#![doc(html_favicon_url = "https://v2.nonebot.dev/logo.png")]
#![doc(html_logo_url = "https://v2.nonebot.dev/logo.png")]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Nonebot-rs 简称 nbrs，是一个基于 Nonebot2 思路的 Onebot 协议机器人框架 Rust 实现。
//! 本框架的基本目标是实现比较便利的 Rust Onebot 机器人搭建。长期目标是以本项目为基础，
//! 开发与其他脚本语言（比如：Python、Lua）互通的 Onebot 机器人平台（如果我能坚持下去
//! 的话）。
//!
//! 基于本框架实现的机器人，可以由一下几部分组成：nbrs 核心、Matcher 插件、启动文件，
//! 每个部分均可独立为单个 crate ，通过启动文件向 nbrs 注册 Matcher 后编译启动的方式
//! 构建一个机器人实例。
//!
//! ## nbrs 设计
//!
//! nbrs 启动后，将读取设置文件、并注册 Matchers（其实这一步已经在编译时硬编码），当接
//! 收到 WebSocket 连接后，加载 Bot 设置，接受 Event 后，由 nbrs 逐级匹配分发到各个
//!  Matcher ，Matcher 处理后，通过 channel 将数据传递回 WebSocket 发送。每个 Event
//! 的匹配与 Matcher 的处理均为独立协程，以此提高并发性能。
//!
//! ## Nonebotrs.toml
//!
//! 当第一次运行 nbrs 时将会自动创建 Nonebotrs.toml 配置文件。
//!
//! ```toml
//! [global]                     # 全局设置
//! host = "127.0.0.1"           # 监听 host
//! port = 8088                  # 监听 port
//! debug = true                 # 开启 debug log
//! superusers = ["YourID"]      # 全局管理员账号
//! nicknames = ["nickname"]     # 全局 Bot 昵称
//! command_starts = ["/"]       # 全局命令起始符
//! access_token = "AccessToken" # 连接鉴权使用
//!
//! [bots.BotID]                 # Bot 设置
//! superusers = ["YourID"]      # 管理员账户
//! nicknames = ["nickname"]     # Bot 昵称
//! command_starts = ["/"]       # 命令起始符
//! access_token = "AccessToken" # 连接鉴权使用
//! ```
//!
//! ## Plugin
//!
//! > To-do
//!
//! ## Matcher
//!
//! **需要启用 feature matcher**
//!
//! 最小运行实例：
//!
//! ```rust
//! fn main() {
//!     let mut nb = nonebot_rs::Nonebot::new(); // 新建 Nonebot
//!
//!     let mut matchers = nonebot_rs::Matchers::new_empty();        // 新建空 Matchers Plugin
//!     matchers
//!         .add_message_matcher(nonebot_rs::builtin::echo::echo())  // 注册 echo Matcher
//!         .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb()); // 注册 rcnb Matcher
//!     nb.add_plugin(scheduler);                                    // 添加 Plugin
//!
//!     nb.run()                                                     // 运行 Nonebot
//! }
//! ```
//!
//! Matcher 开发：
//!
//! ```rust
//! use nonebot_rs::matcher::prelude::*;
//! use rcnb_rs::encode;
//!
//! #[derive(Clone)]   // handler struct 需要生成 Clone trait
//! pub struct Rcnb {} // 定义 handler struct，可以在该结构体容纳静态数据
//!
//! #[async_trait]
//! impl Handler<MessageEvent> for Rcnb {
//!     on_command!(MessageEvent, "rcnb", "RCNB", "Rcnb"); // 注册该 Matcher 的命令匹配器
//!     async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
//!         // 请求获取 msg，event raw_message 为空则发送消息请求消息
//!         let msg = matcher
//!             .request_message(Some(&event), Some("Please enter something."))
//!             .await;
//!         // 再次获取消息依然为空将返回 None
//!         if let Some(msg) = msg {
//!             // 发送处理后的消息
//!             matcher.send_text(&encode(&msg)).await;
//!         }
//!     }
//! }
//!
//! // Matcher 的构建函数
//! pub fn rcnb() -> Matcher<MessageEvent> {
//!     Matcher::new("Rcnb", Rcnb {}) // 声明 Matcher 的 name 与 handler struct
//!         .add_pre_matcher(builtin::prematchers::to_me())         // 添加 to_me prematcher
//!         .add_pre_matcher(builtin::prematchers::command_start()) // 添加 command_start permatcher
//! }
//! ```
//!
//! 使用 Onebot Api：
//!
//! ```rust
//! let msg:Option<nonebot_rs::api_resp::Message> = matcher.get_msg().await
//! ```
//!
//! 没有启用 matcher ？
//!
//! ```rust
//! let msg:Option<nonebot_rs::api_resp::Message> = bot.get_msg().await
//! ```
//!
//! 就是这么简单~
//!
//! ## 定时任务
//!
//! **需要启用 feature scheduler**
//!
//! 定义一个定时任务
//!
//! ```rust
//! use nonebot_rs::{Job, Message};
//!
//! pub fn clock(nb: &nonebot_rs::Nonebot) -> Job {
//!     let bot_getter = nb.bot_getter.clone();
//!     Job::new("1 * * * * *", move |_, _| {
//!         let bots = bot_getter.borrow().clone();
//!         for (_, bot) in bots {
//!             let bot = bot.clone();
//!             tokio::spawn(send_a_msg(bot));
//!         }
//!     })
//!     .unwrap()
//! }
//!
//! // Just for test
//! async fn send_a_msg(bot: nonebot_rs::Bot) {
//!     for superuser in &bot.config.superusers {
//!         bot.send_private_msg(
//!             superuser.parse().unwrap(),
//!             vec![Message::text("One minute passed.")],
//!         )
//!         .await;
//!     }
//! }
//! ```
//!
//! 注册定时任务
//!
//! ```rust
//! use nonebot_rs;
//!
//! fn main() {
//!     let mut nb = nonebot_rs::Nonebot::new();
//!
//!     let mut scheduler = nonebot_rs::Scheduler::new();
//!     scheduler.add_job(clock::clock(&nb));
//!     nb.add_plugin(scheduler);
//!
//!     nb.run()
//! }
//! ```
//!
//! enjoy~

/////////////////////////////////////////////////////////////////////////////////

mod action;
/// Onebot Api
pub mod api;
/// Onebot Api Response
pub mod api_resp;
mod bot;
/// 内建组件
pub mod builtin;
pub mod comms;
/// nbrs 设置项
pub mod config;
/// Onebot 事件
pub mod event;
/// logger
pub mod log;
mod logger;
/// Matchers Plugin
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod matcher;
#[doc(hidden)]
pub mod message;
mod nb;
#[doc(hidden)]
pub mod plugin;
/// scheduler Plugin
#[cfg(feature = "scheduler")]
#[cfg_attr(docsrs, doc(cfg(feature = "scheduler")))]
pub mod scheduler;
mod utils;

use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc, watch};

#[doc(inline)]
pub use action::Action;
#[doc(inline)]
pub use api_resp::{ApiResp, RespData};
pub use async_trait::async_trait;
#[doc(inline)]
pub use bot::Bot;
#[doc(inline)]
#[doc(inline)]
pub use message::Message;
#[doc(inline)]
pub use plugin::Plugin;

#[cfg(feature = "scheduler")]
#[cfg_attr(docsrs, doc(cfg(feature = "scheduler")))]
pub use scheduler::Scheduler;

#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub use matcher::matchers::Matchers;

/// Onebot Api mpsc channel Bot 发送 WebSocket 接收
pub type ApiSender = mpsc::Sender<ApiChannelItem>;
/// Bot 监视 Onebot ApiResp Watch channel
pub type ApiRespWatcher = watch::Receiver<ApiResp>;
/// Event broadcast channel sender 所有 WebSocket Plugin 共享，
/// WebSocket 发送，Plugin 接收
pub type EventSender = broadcast::Sender<event::Event>;
/// Event broadcast channel receiver 所有 WebSocket Plugin 共享，
/// WebSocket 发送，Plugin 接收
pub type EventReceiver = broadcast::Receiver<event::Event>;
/// Nonebot Action Sender，Bot 发送，Nonebot 接收
pub type ActionSender = mpsc::Sender<Action>;
/// Nonebot Action Sender，Bot 发送，Nonebot 接收
pub type ActionReceiver = mpsc::Receiver<Action>;
/// 广播所有可用的 Bot
pub type BotSender = watch::Sender<HashMap<String, Bot>>;
/// 接收广播的所有可用 Bot
pub type BotGetter = watch::Receiver<HashMap<String, Bot>>;
/// nbrs 本体
///
/// 用于注册 `Matcher`，暂存配置项，以及启动实例
pub struct Nonebot {
    /// Nonebot 设置
    pub config: config::NbConfig,
    /// 储存 Nonebot 下连接的 Bot
    pub bots: HashMap<String, Bot>,
    /// 暂存 Events Sender 由 WebSocket 广播 Event
    event_sender: EventSender,
    /// Nonebot Action Sender
    action_sender: ActionSender,
    /// Nonebot Action Receiver
    action_receiver: ActionReceiver,
    /// Bot Sender
    pub bot_sender: BotSender,
    /// Bot Getter
    pub bot_getter: BotGetter,
    /// event handler
    plugins: HashMap<String, Box<dyn Plugin + Send + Sync>>,
}

/// api channel 传递项
#[derive(Debug)]
pub enum ApiChannelItem {
    /// Onebot Api
    Api(api::Api),
    /// Event 用于临时 Matcher 与原 Matcher 传递事件 todo
    MessageEvent(event::MessageEvent),
    /// Time out 通知T
    TimeOut,
}
