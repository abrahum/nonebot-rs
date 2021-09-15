use crate::ApiChannelItem;
use colored::*;
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
                api_resp_watcher,
            } => {
                let bot = self.add_bot(
                    bot_id.clone(),
                    api_sender,
                    action_sender,
                    api_resp_watcher.clone(),
                );
                self.event_sender
                    .send(crate::event::Event::Nonebot(
                        crate::event::NbEvent::BotConnect { bot },
                    ))
                    .unwrap();
                event!(Level::DEBUG, "Add Bot [{}]", bot_id);
            }
            Action::RemoveBot { bot_id } => {
                let bot = self.remove_bot(bot_id.clone());
                match bot {
                    Some(bot) => {
                        event!(Level::DEBUG, "Remove Bot [{}]", bot.bot_id.bright_red());
                        self.event_sender
                            .send(crate::event::Event::Nonebot(
                                crate::event::NbEvent::BotDisconnect { bot },
                            ))
                            .unwrap();
                    }
                    None => {
                        event!(
                            Level::WARN,
                            "Removing not exists Bot [{}]",
                            bot_id.bright_red()
                        );
                    }
                }
            }
            Action::ChangeBotConfig { bot_id, bot_config } => {
                let bot = self.bots.get_mut(&bot_id).unwrap();
                bot.config = bot_config;
            }
        }
    }
}
