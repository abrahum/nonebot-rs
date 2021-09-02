use crate::{api, api_resp, config, message, utils, ApiChannelItem, ApiResp};
use tokio::sync::{mpsc, watch};

/// Bot
#[derive(Debug, Clone)]
pub struct Bot {
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
            .send(ApiChannelItem::Api(api::Api::SendGroupMsg {
                params: api::SendGroupMsg {
                    group_id: group_id,
                    message: msg.clone(),
                    auto_escape: false,
                },
                echo: format!("{}-{:?}", self.config.bot_id, msg),
            }))
            .await
            .unwrap();
    }

    /// Send Private Msg
    pub async fn send_private_msg(&self, user_id: i64, msg: Vec<message::Message>) {
        self.api_sender
            .send(ApiChannelItem::Api(api::Api::SendPrivateMsg {
                params: api::SendPrivateMsg {
                    user_id: user_id,
                    message: msg.clone(),
                    auto_escape: false,
                },
                echo: format!("{}-{:?}", self.config.bot_id, msg),
            }))
            .await
            .unwrap();
    }

    /// same as Matcher
    pub async fn call_api(&self, api: api::Api) {
        self.api_sender
            .send(ApiChannelItem::Api(api))
            .await
            .unwrap();
    }

    /// same as Matcher
    pub async fn call_api_resp(&mut self, api: api::Api) -> Option<api_resp::ApiResp> {
        let echo = api.get_echo();
        self.api_sender
            .send(ApiChannelItem::Api(api))
            .await
            .unwrap();
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
