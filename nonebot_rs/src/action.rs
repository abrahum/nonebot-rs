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
    RemoveBot { bot_id: String },
    /// 变更 BotConfig
    ChangeBotConfig {
        bot_id: String,
        bot_config: crate::config::BotConfig,
    },
}

impl crate::Nonebot {
    /// 处理 Nonebot 内部 Action
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
            Action::ChangeBotConfig { bot_id, bot_config } => {
                let bot = self.bots.get_mut(&bot_id).unwrap();
                bot.config = bot_config;
            }
        }
    }
}
