use crate::{api_resp, config, message, utils, ApiChannelItem, ApiResp};
use colored::*;
use tokio::sync::{mpsc, watch};
use tracing::{event, Level};

/// Bot
#[derive(Debug, Clone)]
pub struct Bot {
    /// connect timestamp
    pub connect_time: i64,
    // Bot Config
    pub config: config::BotConfig,
    /// 暂存调用 Bot api
    pub api_sender: mpsc::Sender<ApiChannelItem>,
    /// 暂存 ApiResp Receiver
    pub api_resp_watcher: watch::Receiver<ApiResp>,
}

impl Bot {
    /// Send Group Msg
    pub async fn send_group_msg(&self, group_id: i64, msg: Vec<message::Message>) {
        self.api_sender
            .send(ApiChannelItem::Api(crate::Api::send_group_msg(
                crate::SendGroupMsg {
                    group_id: group_id,
                    message: msg.clone(),
                    auto_escape: false,
                },
            )))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Send {:?} to Group ({})",
            self.config.bot_id.red(),
            msg,
            group_id.to_string().magenta()
        );
    }

    /// Send Private Msg
    pub async fn send_private_msg(&self, user_id: i64, msg: Vec<message::Message>) {
        self.api_sender
            .send(ApiChannelItem::Api(crate::Api::send_private_msg(
                crate::SendPrivateMsg {
                    user_id: user_id,
                    message: msg.clone(),
                    auto_escape: false,
                },
            )))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Send {:?} to Friend ({})",
            self.config.bot_id.red(),
            msg,
            user_id.to_string().green()
        );
    }

    /// same as Matcher
    pub async fn call_api(&self, api: crate::Api) {
        self.api_sender
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.config.bot_id.red(),
            api
        );
    }

    /// same as Matcher
    pub async fn call_api_resp(&mut self, api: crate::Api) -> Option<api_resp::ApiResp> {
        let echo = api.get_echo();
        self.api_sender
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.config.bot_id.red(),
            api
        );
        let time = utils::timestamp();
        while let Ok(_) = self.api_resp_watcher.changed().await {
            let resp = self.api_resp_watcher.borrow().clone();
            if resp.echo == echo {
                return Some(resp);
            }
            if utils::timestamp() > time + 30 {
                return None;
            }
        }
        None
    }
}
