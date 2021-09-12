use crate::ApiChannelItem;
use tokio::sync::{mpsc, watch};
use tracing::{event, Level};

/// Nonebot 内部设置项
#[derive(Debug, Clone)]
pub enum Action {
    /// 添加 Bot
    AddBot {
        bot_id: String,
        api_sender: mpsc::Sender<ApiChannelItem>,
        action_sender: crate::ActionSender,
        auth: Option<String>,
        api_resp_watcher: watch::Receiver<crate::api_resp::ApiResp>,
    },
    /// 移除 Bot
    RemoveBot {
        bot_id: String,
    },
    // /// 移除 Matcher
    // #[cfg(feature = "matcher")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    RemovePlugin {
        plugin_name: String,
    },
    // AddPlugin {
    //     plugin_name: String,
    //     plugin: std::sync::Arc<(dyn crate::Plugin + Send + Sync + 'static)>,
    // },
    // /// 添加 Matcher<MessageEvent>
    // ///
    // /// 当存在同名 Matcher 时，将会替代旧 Matcher
    // #[cfg(feature = "matcher")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    // AddMessageEventMatcher {
    //     message_event_matcher: crate::matcher::Matcher<crate::event::MessageEvent>,
    // },
    // /// 禁用 Matcher
    // #[cfg(feature = "matcher")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    // DisableMatcher { name: String },
    // /// 取消禁用 Matcher
    // #[cfg(feature = "matcher")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    // EnableMatcher { name: String },
    /// 变更 BotConfig
    ChangeBotConfig {
        bot_id: String,
        bot_config: crate::config::BotConfig,
    },
}

impl crate::Nonebot {
    /// 处理 Nonebot 内部设置项
    pub fn handle_action(&mut self, action: Action) {
        event!(Level::DEBUG, "Receive Action {:?}", action);
        match action {
            Action::AddBot {
                bot_id,
                api_sender,
                action_sender,
                auth,
                api_resp_watcher,
            } => {
                if crate::Nonebot::check_auth(auth) {
                    self.add_bot(
                        bot_id.clone(),
                        api_sender,
                        action_sender,
                        api_resp_watcher.clone(),
                    );
                    event!(Level::DEBUG, "Add Bot [{}]", bot_id);
                } else {
                    event!(Level::WARN, "Bot [{}] authorize failure", bot_id);
                }
            }
            Action::RemoveBot { bot_id } => {
                self.remove_bot(bot_id.clone());
                event!(Level::DEBUG, "Remove Bot [{}]", bot_id);
            }
            Action::RemovePlugin { plugin_name } => self.remove_plugin(&plugin_name),
            // Action::AddPlugin {
            //     plugin_name,
            //     plugin,
            // } => self.add_plugin(&plugin_name, plugin),
            // #[cfg(feature = "matcher")]
            // Action::AddMessageEventMatcher {
            //     message_event_matcher: matcher,
            // } => {
            //     self.matchers.add_message_matcher(matcher.clone());
            //     event!(Level::DEBUG, "Add messageevent matcher {:?}", matcher);
            // }
            // #[cfg(feature = "matcher")]
            // Action::RemoveMatcher { name: matcher_name } => {
            //     self.matchers.remove_matcher(&matcher_name);
            //     event!(Level::DEBUG, "Remove matcher {}", matcher_name);
            // }
            // #[cfg(feature = "matcher")]
            // Action::DisableMatcher { name: matcher_name } => {
            //     self.matchers.disable_matcher(&matcher_name, true);
            //     event!(Level::DEBUG, "Disable matcher{}", matcher_name);
            // }
            // #[cfg(feature = "matcher")]
            // Action::EnableMatcher { name: matcher_name } => {
            //     self.matchers.disable_matcher(&matcher_name, false);
            //     event!(Level::DEBUG, "Enable matcher{}", matcher_name);
            // }
            Action::ChangeBotConfig { bot_id, bot_config } => {
                let bot = self.bots.get_mut(&bot_id).unwrap();
                bot.config = bot_config;
            }
        }
    }
}
