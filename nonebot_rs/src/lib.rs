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
//! 收到 WebSocket 连接后，加载 Bot 设置，接受 Event 后，由 nbrs 逐级匹配分发到各个
//!  Matcher ，Matcher 处理后，通过 channel 将数据传递回 WebSocket 发送。每个 Event
//! 的匹配与 Matcher 的处理均为独立协程，以此提高并发性能。
//!
//! # Nonebotrs.toml
//!
//! 当第一次运行 nbrs 时将会自动创建 Nonebotrs.toml 配置文件。
//!
//! ```toml
//! [global]                 // 全局设置
//! host = "127.0.0.1"       // 监听 host
//! port = 8088              // 监听 port
//! debug = true             // 开启 debug log
//! superusers = ["YourID"]  // 全局管理员账号
//! nicknames = ["nickname"] // 全局 Bot 昵称
//! command_starts = ["/"]   // 全局命令起始符
//!
//! [bots.BotID]             // Bot 设置
//! superusers = ["YourID"]  // 管理员账户
//! nicknames = ["nickname"] // Bot 昵称
//! command_starts = ["/"]   // 命令起始符
//! ```
//!
//! # Examples
//!
//! 最小运行实例：
//!
//! ```rust
//! fn main() {
//!     let mut nb = nonebot_rs::Nonebot::new(); // 新建 Nonebot
//!     nb.matchers
//!         .add_message_matcher(nonebot_rs::builtin::echo::echo())  // 注册 echo Matcher
//!         .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb()); // 注册 rcnb Matcher
//!     nb.run()                                                     // 运行 Nonebot
//! }
//! ```
//!
//! Matcher 开发：
//!
//! ```rust
//! use nonebot_rs::builtin;
//! use nonebot_rs::event::MessageEvent;
//! use nonebot_rs::matcher::{Handler, Matcher};
//! use nonebot_rs::on_command;
//! use nonebot_rs::async_trait;
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

/////////////////////////////////////////////////////////////////////////////////

mod action;
/// Onebot Api
pub mod api;
#[doc(hidden)]
mod api_resp;
#[doc(hidden)]
pub mod axum_driver;
mod bot;
/// 内建组件
pub mod builtin;
/// nbrs 设置项
pub mod config;
/// Onebot 事件
pub mod event;
/// logger
pub mod log;
/// Matcher 定义
#[cfg(feature = "matcher")]
pub mod matcher;
#[doc(hidden)]
pub mod message;
mod plugin;
#[doc(hidden)]
#[cfg(feature = "pyo")]
pub mod pyo;
mod utils;

use std::collections::HashMap;
use tokio::sync::{mpsc, watch};
#[cfg(feature = "scheduler")]
pub use tokio_cron_scheduler::Job;
#[cfg(feature = "scheduler")]
use tokio_cron_scheduler::JobScheduler;

#[doc(inline)]
pub use action::Action;
pub use api::*;
#[doc(inline)]
pub use api_resp::{ApiResp, RespData};
pub use async_trait::async_trait;
#[doc(inline)]
pub use bot::Bot;
#[doc(inline)]
#[cfg(feature = "matcher")]
pub use matcher::matchers::{Matchers, MatchersBTreeMap, MatchersHashMap};
#[doc(inline)]
pub use message::Message;

#[macro_use]
extern crate lazy_static;

pub type ApiSender = mpsc::Sender<ApiChannelItem>;
pub type ApiRespWatcher = watch::Receiver<ApiResp>;

/// nbrs 本体
///
/// 用于注册 `Matcher`，暂存配置项，以及启动实例
pub struct Nonebot {
    /// Nonebot 设置
    pub config: config::NbConfig,
    /// 储存 Nonebot 下连接的 Bot
    pub bots: HashMap<String, Bot>,
    /// 暂存 Events Sender
    event_sender: mpsc::Sender<EventChannelItem>,
    /// Events Receiver
    event_receiver: mpsc::Receiver<EventChannelItem>,
    /// Bot Sender
    pub bot_sender: watch::Sender<HashMap<String, Bot>>,
    /// Bot Getter
    pub bot_getter: watch::Receiver<HashMap<String, Bot>>,
    #[cfg(feature = "matcher")]
    pub matchers: Matchers,
    #[cfg(feature = "scheduler")]
    pub scheduler: JobScheduler,
}

/// api channel 传递项
#[derive(Debug)]
pub enum ApiChannelItem {
    Action(Action),
    /// Onebot Api
    Api(api::Api),
    /// Event 用于临时 Matcher 与原 Matcher 传递事件 todo
    MessageEvent(event::MessageEvent),
    /// Timeout
    TimeOut,
}

/// evnet channel 传递项
#[derive(Debug)]
pub enum EventChannelItem {
    Action(Action),
    Event(event::Event),
}

impl Nonebot {
    /// 当 WenSocket 收到配置中未配置的 Bot 时，调用该方法新建 Bot 配置信息
    pub fn add_bot(
        &mut self,
        bot_id: i64,
        api_sender: mpsc::Sender<ApiChannelItem>,
        api_resp_watcher: watch::Receiver<ApiResp>,
    ) {
        let bot_id = bot_id.to_string();
        self.bots.insert(
            bot_id.clone(),
            Bot {
                config: self.config.gen_bot_config(&bot_id),
                api_sender: api_sender,
                api_resp_watcher: api_resp_watcher,
            },
        );
        self.bot_sender.send(self.bots.clone()).unwrap();
    }

    pub fn remove_bot(&mut self, bot_id: i64) {
        let bot_id = bot_id.to_string();
        self.bots.remove(&bot_id).expect("");
        self.bot_sender.send(self.bots.clone()).unwrap();
    }

    fn check_auth(_auth: Option<String>) -> bool {
        // todo
        true
    }

    /// 新建一个 Matchers 为空的 Nonebot 结构体
    pub fn new() -> Self {
        let nb_config = config::NbConfig::load();
        let (event_sender, event_recevier) = tokio::sync::mpsc::channel(32);
        let (bot_sender, bot_getter) = watch::channel(HashMap::new());
        Nonebot {
            bots: HashMap::new(),
            config: nb_config,
            event_sender: event_sender,
            event_receiver: event_recevier,
            bot_sender: bot_sender,
            bot_getter: bot_getter,
            #[cfg(feature = "matcher")]
            matchers: Matchers::new(None, None, None, None),
            #[cfg(feature = "scheduler")]
            scheduler: JobScheduler::new(),
        }
    }

    #[doc(hidden)]
    pub fn pre_run(&self) {
        use colored::*;
        log::init(self.config.global.debug, self.config.global.trace);
        tracing::event!(
            tracing::Level::INFO,
            "{}",
            "高性能自律実験4号機が稼働中····".red()
        );
    }

    /// Nonebot EventChannel receive handle
    async fn recv(mut self) {
        while let Some(event_channel_item) = self.event_receiver.recv().await {
            match event_channel_item {
                EventChannelItem::Action(action) => self.handle_action(action),
                EventChannelItem::Event(e) => self.handle_event(e),
            }
        }
    }

    /// Nonebot Event handle
    fn handle_event(&mut self, e: event::Event) {
        tracing::event!(tracing::Level::TRACE, "handling events {:?}", e);
        match &e {
            event::Event::Message(e) => {
                builtin::logger(&e);
            }
            event::Event::Meta(e) => {
                builtin::metahandle(&e);
            }
            _ => {}
        }

        #[cfg(feature = "matcher")]
        {
            use event::SelfId;
            let bot = self.bots.get(&e.get_self_id()).unwrap();
            self.matchers.handle_events(
                e,
                bot.config.clone(),
                bot.api_sender.clone(),
                bot.api_resp_watcher.clone(),
            )
        }
    }

    /// 运行 Nonebot 实例
    #[tokio::main]
    pub async fn run(self) {
        self.pre_run();
        tokio::spawn(axum_driver::run(
            self.config.global.host,
            self.config.global.port,
            self.event_sender.clone(),
        ));
        #[cfg(feature = "scheduler")]
        tokio::spawn(self.scheduler.start());
        self.recv().await;
    }

    #[doc(hidden)]
    pub async fn async_run(self) {
        self.pre_run();
        tokio::spawn(axum_driver::run(
            self.config.global.host,
            self.config.global.port,
            self.event_sender.clone(),
        ));
        self.recv().await;
    }
}
