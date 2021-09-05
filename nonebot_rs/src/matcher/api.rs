use super::Matcher;
use crate::event::SelfId;
use crate::ApiChannelItem;
use colored::*;
use tracing::{event, Level};

impl<E> Matcher<E>
where
    E: Clone + SelfId,
{
    /// 请求 Onebot Api，不等待 Onebot 返回
    pub async fn call_api(&self, api: crate::Api) {
        self.api_sender
            .clone()
            .unwrap()
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.event.clone().unwrap().get_self_id().red(),
            api
        );
    }

    /// 请求 Onebot Api，等待 Onebot 返回项（30s 后 timeout 返回 None）
    pub async fn call_api_resp(&self, api: crate::Api) -> Option<crate::api_resp::ApiResp> {
        let echo = api.get_echo();
        self.api_sender
            .clone()
            .unwrap()
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.event.clone().unwrap().get_self_id().red(),
            api
        );
        let mut watcher = self.api_resp_watcher.clone().unwrap();
        let time = crate::utils::timestamp();
        while let Ok(_) = watcher.changed().await {
            let resp = (*watcher.borrow()).clone();
            if resp.echo == echo {
                return Some(resp.clone());
            }
            if crate::utils::timestamp() > time + 30 {
                return None;
            }
        }
        None
    }
}
