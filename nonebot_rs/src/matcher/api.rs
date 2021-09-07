use super::Matcher;
use crate::event::SelfId;
use colored::*;
use tracing::{event, Level};

impl<E> Matcher<E>
where
    E: Clone + SelfId,
{
    /// 请求 Onebot Api，不等待 Onebot 返回
    pub async fn call_api(&self, api: crate::Api) {
        if let Some(bot) = &self.bot {
            bot.call_api(api).await;
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Calling api with unbuilt matcher!".red()
            );
        }
    }

    /// 请求 Onebot Api，等待 Onebot 返回项（30s 后 timeout 返回 None）
    pub async fn call_api_resp(&self, api: crate::Api) -> Option<crate::api_resp::ApiResp> {
        if let Some(bot) = &self.bot {
            bot.call_api_resp(api).await
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Calling api with unbuilt matcher!".red()
            );
            None
        }
    }
}
