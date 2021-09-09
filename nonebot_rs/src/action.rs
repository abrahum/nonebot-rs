use crate::ApiChannelItem;
use tokio::sync::{mpsc, watch};
use tracing::{event, Level};

/// Nonebot 内部设置项
#[derive(Debug, Clone)]
pub enum Action {
    /// 添加 Bot
    AddBot {
        bot_id: i64,
        api_sender: mpsc::Sender<ApiChannelItem>,
        auth: Option<String>,
        api_resp_watcher: watch::Receiver<crate::api_resp::ApiResp>,
    },
    /// 移除 Bot
    RemoveBot { bot_id: i64 },
    /// 移除 Matcher
    #[cfg(feature = "matcher")]
    #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    RemoveMatcher { name: String },
    /// 添加 Matcher<MessageEvent>
    ///
    /// 当存在同名 Matcher 时，将会替代旧 Matcher
    #[cfg(feature = "matcher")]
    #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    AddMessageEventMatcher {
        message_event_matcher: crate::matcher::Matcher<crate::event::MessageEvent>,
    },
    /// 禁用 Matcher
    #[cfg(feature = "matcher")]
    #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    DisableMatcher { name: String },
    /// 取消禁用 Matcher
    #[cfg(feature = "matcher")]
    #[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
    EnableMatcher { name: String },
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
                auth,
                api_resp_watcher,
            } => {
                if crate::Nonebot::check_auth(auth) {
                    let bot = self.add_bot(bot_id, api_sender.clone(), api_resp_watcher.clone());
                    event!(Level::DEBUG, "Add Bot [{}]", bot_id);
                    #[cfg(feature = "matcher")]
                    self.matchers.run_on_connect(bot);
                } else {
                    event!(Level::WARN, "Bot [{}] authorize failure", bot_id);
                }
            }
            Action::RemoveBot { bot_id } => {
                self.remove_bot(bot_id);
                event!(Level::DEBUG, "Remove Bot [{}]", bot_id);
            }
            #[cfg(feature = "matcher")]
            Action::AddMessageEventMatcher {
                message_event_matcher: matcher,
            } => {
                self.matchers.add_message_matcher(matcher.clone());
                event!(Level::DEBUG, "Add messageevent matcher {:?}", matcher);
            }
            #[cfg(feature = "matcher")]
            Action::RemoveMatcher { name: matcher_name } => {
                self.matchers.remove_matcher(&matcher_name);
                event!(Level::DEBUG, "Remove matcher {}", matcher_name);
            }
            #[cfg(feature = "matcher")]
            Action::DisableMatcher { name: matcher_name } => {
                self.matchers.disable_matcher(&matcher_name, true);
                event!(Level::DEBUG, "Disable matcher{}", matcher_name);
            }
            #[cfg(feature = "matcher")]
            Action::EnableMatcher { name: matcher_name } => {
                self.matchers.disable_matcher(&matcher_name, false);
                event!(Level::DEBUG, "Enable matcher{}", matcher_name);
            }
            Action::ChangeBotConfig { bot_id, bot_config } => {
                let bot = self.bots.get_mut(&bot_id).unwrap();
                bot.config = bot_config;
            }
        }
    }
}
