use crate::api::Apis;
use crate::butin;
use crate::event::Events;
use crate::{Matchers, Nonebot};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

pub type ApiSender = Sender<Apis>;

pub struct Bot {
    self_id: String,           // bot ID
    amnb: Arc<Mutex<Nonebot>>, // Nonebot
    sender: ApiSender,         // channel sender
    matchers: Matchers,
}

impl Bot {
    pub fn new(
        id: i64,
        authorization: Option<String>,
        sender: Sender<Apis>,
        amnb: Arc<Mutex<Nonebot>>,
    ) -> Result<Self, String> {
        Bot::check_auth(authorization, amnb.clone())?;
        let matchers: Matchers;
        {
            let nb = amnb.lock().unwrap();
            matchers = nb.matchers.clone();
        }
        let bot = Bot {
            self_id: id.to_string(),
            amnb: amnb,
            sender: sender,
            matchers: matchers,
        };
        Ok(bot)
    }

    #[allow(dead_code)]
    pub fn get_self_id(&self) -> &str {
        &self.self_id
    }

    pub async fn handle_recv(&self, msg: String) {
        let data: serde_json::error::Result<Events> = serde_json::from_str(&msg);
        match data {
            Ok(events) => self.handle_events(events).await,
            Err(_) => self.handle_resp(msg).await,
        }
        match self.sender.try_send(crate::api::Apis::None) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    async fn handle_events(&self, events: Events) {
        match events {
            Events::Message(e) => {
                butin::logger(&e).await.unwrap();
                self.handle_event(&self.matchers.message, e).await;
            }
            Events::Notice(e) => {
                self.handle_event(&self.matchers.notice, e).await;
            }
            Events::Request(e) => self.handle_event(&self.matchers.request, e).await,
            Events::Meta(e) => {
                butin::metahandle(&e).await;
                self.handle_event(&self.matchers.meta, e).await;
            }
        }
    }

    async fn handle_resp(&self, resp: String) {
        let resp: crate::api::ApiResp = serde_json::from_str(&resp).unwrap();
        butin::resp_logger(resp).await;
    }

    async fn handle_event<E>(&self, matchers: &crate::MatchersVec<E>, e: E)
    where
        E: Clone + Send + 'static,
    {
        for matcher in matchers {
            matcher
                .match_(e.clone(), self.amnb.clone(), self.sender.clone())
                .await
                .unwrap();
        }
    }

    fn check_auth(auth: Option<String>, amnb: Arc<Mutex<Nonebot>>) -> Result<bool, String> {
        //todo
        Ok(true)
    }
}
