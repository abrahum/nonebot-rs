use crate::{ActionSender, ApiChannelItem, ApiResp, Bot, Nonebot, Plugin};
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc, watch};

impl Nonebot {
    /// 当 WenSocket 收到配置中未配置的 Bot 时，调用该方法新建 Bot 配置信息
    pub fn add_bot(
        &mut self,
        bot_id: String,
        api_sender: mpsc::Sender<ApiChannelItem>,
        action_sender: ActionSender,
        api_resp_watcher: watch::Receiver<ApiResp>,
    ) -> Bot {
        let bot = Bot::new(
            bot_id.clone(),
            self.config.gen_bot_config(&bot_id),
            api_sender,
            action_sender,
            api_resp_watcher,
        );
        self.bots.insert(bot_id.to_string(), bot.clone());
        self.bot_sender.send(self.bots.clone()).unwrap();
        bot
    }

    /// 移除 Bot，移除成功则返回移除的 Bot
    pub fn remove_bot(&mut self, bot_id: String) -> Option<Bot> {
        let bot_id = bot_id.to_string();
        let bot = self.bots.remove(&bot_id);
        self.bot_sender.send(self.bots.clone()).unwrap();
        bot
    }

    /// 新建一个 Matchers 为空的 Nonebot 结构体
    pub fn new() -> Self {
        let nb_config = crate::config::NbConfig::load();
        let (event_sender, _) = broadcast::channel(1024); // need largo cache when reconnect
        let (action_sender, action_receiver) = tokio::sync::mpsc::channel(32);
        let (bot_sender, bot_getter) = watch::channel(HashMap::new());
        Nonebot {
            bots: HashMap::new(),
            config: nb_config,
            event_sender,
            action_sender,
            action_receiver,
            bot_sender,
            bot_getter,
            plugins: HashMap::new(),
        }
    }

    /// 添加 Plugin
    pub fn add_plugin<P>(&mut self, p: P)
    where
        P: Plugin + Send + Sync + 'static,
    {
        self.plugins.insert(p.plugin_name().to_owned(), Box::new(p));
    }

    /// 移除 Plugin
    pub fn remove_plugin(&mut self, plugin_name: &str) {
        self.plugins.remove(plugin_name);
    }

    #[doc(hidden)]
    pub async fn pre_run(&mut self) {
        use colored::*;
        crate::log::init(self.config.global.debug, self.config.global.trace);
        tracing::event!(tracing::Level::INFO, "Loaded Config {:?}", self.config);
        tracing::event!(
            tracing::Level::DEBUG,
            "Full Config {:?}",
            self.config.get_full_config()
        );
        tracing::event!(
            tracing::Level::INFO,
            "{}",
            "高性能自律実験4号機が稼働中····".red()
        );
        self.add_plugin(crate::logger::Logger);
        for (plugin_name, plugin) in &mut self.plugins {
            let plugin_config: Option<toml::Value> =
                self.config.get_config(&plugin.plugin_name().to_lowercase());
            if let Some(plugin_config) = plugin_config {
                plugin.load_config(plugin_config).await;
            }
            plugin.run(self.event_sender.subscribe(), self.bot_getter.clone());
            tracing::event!(
                tracing::Level::INFO,
                "Plugin {} is loaded.",
                plugin_name.red()
            );
        }
    }

    /// Nonebot EventChannel receive handle
    async fn recv(mut self) {
        while let Some(action) = self.action_receiver.recv().await {
            self.handle_action(action)
        }
    }

    /// 运行 Nonebot 实例
    #[tokio::main]
    pub async fn run(self) {
        self.async_run().await;
    }

    #[doc(hidden)]
    pub async fn async_run(mut self) {
        self.pre_run().await;
        // let access_tokens = self.config.gen_access_token();
        // tokio::spawn(crate::comms::revs_ws::run(
        //     self.config.global.host,
        //     self.config.global.port,
        //     self.event_sender.clone(),
        //     self.action_sender.clone(),
        //     access_tokens.clone(),
        // ));
        // tokio::spawn(crate::comms::ws::run(
        //     vec!["ws://127.0.0.1:6700/ws"],
        //     self.event_sender.clone(),
        //     self.action_sender.clone(),
        //     access_tokens,
        // ));
        crate::comms::strat_comms(&self).await;
        self.recv().await;
    }
}
