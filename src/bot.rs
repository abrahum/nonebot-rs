use crate::api::Apis;
use crate::butin;
use crate::event::Events;
use crate::matcher::MatchersVec;
use crate::Nonebot;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

pub type ApiSender = Sender<Apis>;

pub struct Bot {
    self_id: String,           // bot ID
    amnb: Arc<Mutex<Nonebot>>, // Nonebot
    sender: ApiSender,         // channel sender
    superusers: Vec<String>,   // Superusers
    nickname: Vec<String>,     // nickname
    pub command_start: Vec<String>, // command_start

                               // message_event_matchers:, // 按照优先级存储 Matcher
                               // notice_event_matchers: MatcherMap<NoticeEvent>,   // 按照优先级存储 Matcher
                               // request_event_matchers: MatcherMap<RequestEvent>, // 按照优先级存储 Matcher
                               // meta_event_matchers: MatcherMap<MetaEvent>,       // 按照优先级存储 Matcher
}

impl Bot {
    fn load_config(&mut self) {
        let nb = &*(self.amnb).lock().unwrap();
        let bot_config = nb.config.get_bot_config(&self.self_id);
        let global_config = &nb.config.global;

        if let Some(self_config) = bot_config {
            if let Some(superusers) = &self_config.superusers {
                self.superusers = superusers.clone()
            }
        } else if let Some(superusers) = &global_config.superusers {
            self.superusers = superusers.clone()
        }

        if let Some(self_config) = bot_config {
            if let Some(nickname) = &self_config.nickname {
                self.nickname = nickname.clone()
            }
        }

        if let Some(self_config) = bot_config {
            if let Some(command_start) = &self_config.command_start {
                self.command_start = command_start.clone()
            }
        } else if let Some(command_start) = &global_config.command_start {
            self.command_start = command_start.clone()
        }
    }

    pub fn new(
        id: i64,
        authorization: Option<String>,
        sender: Sender<Apis>,
        amnb: Arc<Mutex<Nonebot>>,
    ) -> Self {
        // check authorization here
        // let message_event_matchers: MatcherMap<MessageEvent>;
        // let notice_event_matchers: MatcherMap<NoticeEvent>;
        // let request_event_matchers: MatcherMap<RequestEvent>;
        // let meta_event_matchers: MatcherMap<MetaEvent>;
        {
            // let nb = amnb.lock().unwrap();
            // message_event_matchers = nb.message_event_matchers;
            // notice_event_matchers = nb.notice_event_matchers;
            // request_event_matchers = nb.request_event_matchers;
            // meta_event_matchers = nb.meta_event_matchers;
        }
        let mut bot = Bot {
            self_id: id.to_string(),
            amnb: amnb,
            sender: sender,
            superusers: vec![],
            nickname: vec![],
            command_start: vec![],
            // message_event_matchers: message_event_matchers,
            // notice_event_matchers: notice_event_matchers,
            // request_event_matchers: request_event_matchers,
            // meta_event_matchers: meta_event_matchers,
        };
        bot.load_config();
        bot
    }

    #[allow(dead_code)]
    pub fn get_self_id(&self) -> &str {
        &self.self_id
    }

    pub async fn handle_event(&mut self, msg: String) {
        let eventr: serde_json::error::Result<Events> = serde_json::from_str(&msg);
        match eventr {
            Ok(e) => match e {
                Events::Message(e) => {
                    butin::logger(e.clone()).await.unwrap();
                    // butin::echo::echo(e.clone(), self.sender.clone())
                    //     .await
                    //     .unwrap();
                    butin::echo_::builder()
                        .match_(e.clone(), self.amnb.clone(), self.sender.clone())
                        .await
                        .unwrap();
                }
                Events::Notice(_) => {}
                Events::Request(_) => {}
                Events::Meta(e) => butin::metahandle(e).await,
            },
            Err(_) => {
                let rdata: crate::api::ApiResp = serde_json::from_str(&msg).unwrap();
                butin::resp_logger(rdata).await;
            }
        }
        match self.sender.try_send(crate::api::Apis::None) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    // async fn handle_event_<T>(&mut self, event: &T, matchers: MatcherMap<T>) {}

    //fn check_auth(&self) -> bool {}
    //todo
}
