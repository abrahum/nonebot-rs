use super::Matcher;
use crate::bot::ChannelItem;

impl<E> Matcher<E>
where
    E: Clone,
{
    pub async fn call_api(&self, api: crate::api::Api) {
        self.sender
            .clone()
            .unwrap()
            .send(ChannelItem::Api(api))
            .await
            .unwrap();
    }

    pub async fn call_api_resp(&self, api: crate::Api) -> Option<crate::api_resp::ApiResp> {
        let echo = api.get_echo();
        self.sender
            .clone()
            .unwrap()
            .send(crate::bot::ChannelItem::Api(api))
            .await
            .unwrap();
        let mut watcher = self.watcher.clone().unwrap();
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
