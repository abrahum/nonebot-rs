use crate::{event::Event, ActionSender, EventSender};
use async_recursion::async_recursion;
use colored::*;
use futures_util::{stream::SplitStream, SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::{broadcast, watch},
};
use tokio_tungstenite::{tungstenite::Message as TuMessage, WebSocketStream};
use tracing::{event, Level};

pub async fn handler_web_socket(
    socket: WebSocketStream<TcpStream>,
    event_sender: EventSender,
    action_sender: ActionSender,
    apiresp_watch_sender: tokio::sync::watch::Sender<crate::ApiResp>,
    mut api_receiver: tokio::sync::mpsc::Receiver<crate::ApiChannelItem>,
    bot_id: String,
) {
    // 将 websocket 接收流与发送流分离
    let (mut sink, mut stream) = socket.split();
    // 接收消息
    let another_event_sender = event_sender.clone();
    let income = async move {
        loop {
            let r = stream_recv(
                stream,
                &another_event_sender,
                &action_sender,
                &apiresp_watch_sender,
                bot_id.clone(),
            )
            .await;
            if let Some(s) = r {
                stream = s;
            } else {
                return;
            }
        }
    };
    // 发送消息
    let outcome = async move {
        while let Some(data) = api_receiver.recv().await {
            match data {
                // Onebot Api
                crate::ApiChannelItem::Api(api) => {
                    let json_string = serde_json::to_string(&api).unwrap();
                    sink.send(TuMessage::text(json_string)).await.unwrap();
                }
                // temp Matcher event
                crate::ApiChannelItem::MessageEvent(_) => {
                    event!(
                        Level::WARN,
                        "{}",
                        "WedSocket接受端接收到错误Event消息".bright_red()
                    );
                }
                // temp Matcher Timeout
                crate::ApiChannelItem::TimeOut => {
                    event!(
                        Level::WARN,
                        "{}",
                        "WedSocket接受端接收到错误TimeOut消息".bright_red()
                    );
                } // 忽视 event 该 receiver 永不应该收到 event
            }
        }
    };
    tokio::spawn(income);
    outcome.await;
}

async fn stream_recv(
    stream: SplitStream<WebSocketStream<TcpStream>>,
    event_sender: &EventSender,
    action_sender: &ActionSender,
    apiresp_watch_sender: &watch::Sender<crate::api_resp::ApiResp>,
    bot_id: String,
) -> Option<SplitStream<WebSocketStream<TcpStream>>> {
    let (msg, next_stream) = stream.into_future().await;
    if let Some(msg) = msg {
        use crate::event::RecvItem;
        if let Ok(msg) = msg {
            let data: serde_json::Result<RecvItem> = serde_json::from_str(msg.to_text().unwrap());
            match data {
                Ok(data) => match data {
                    RecvItem::Event(event) => send_event(&event_sender, event).await,
                    RecvItem::ApiResp(api_resp) => {
                        apiresp_watch_sender.send(api_resp).unwrap();
                    }
                },
                Err(e) => {
                    event!(
                        Level::ERROR,
                        "Serialize msg failed! Msg:{:?}\nError:{}",
                        msg.to_text().unwrap(),
                        e
                    );
                    std::process::exit(101);
                }
            }
        } else {
            event!(Level::WARN, "Bot [{}] disconnect", bot_id.to_string().red());
            action_sender
                .send(crate::Action::RemoveBot { bot_id: bot_id })
                .await
                .unwrap();
            return None;
        }
    }
    Some(next_stream)
}

#[async_recursion]
pub async fn send_event(sender: &broadcast::Sender<Event>, e: Event) -> () {
    match sender.send(e) {
        Ok(_) => (),
        Err(broadcast::error::SendError(_)) => {
            event!(Level::ERROR, "EventChannel is full out of cache!");
            std::process::exit(101);
        }
    }
}
