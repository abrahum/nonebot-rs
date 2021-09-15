use crate::{ActionSender, EventSender};
use axum::{
    extract::TypedHeader,
    prelude::*,
    ws::{ws, WebSocket},
};
use colored::*;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, watch};
use tracing::{event, Level};

mod xheaders;

pub async fn run(
    host: std::net::Ipv4Addr,
    port: u16,
    event_sender: EventSender,
    action_sender: ActionSender,
) {
    let handle_socket =
        |socket: WebSocket,
         x_self_id: Option<TypedHeader<xheaders::XSelfId>>,
         x_client_role: Option<TypedHeader<xheaders::XClientRole>>,
         user_agent: Option<TypedHeader<headers::UserAgent>>,
         authorization: Option<TypedHeader<xheaders::Authorization>>| async move {
            event!(Level::INFO, "A client is connected.");
            if let (
                Some(TypedHeader(user_agent)),
                Some(TypedHeader(x_self_id)),
                Some(TypedHeader(x_client_role)),
            ) = (user_agent, x_self_id, x_client_role)
            {
                let (sender, receiver) = mpsc::channel(32);
                let (apiresp_watch_sender, api_resp_watcher) =
                    watch::channel(crate::api_resp::ApiResp {
                        status: "init".to_string(),
                        retcode: 0,
                        data: crate::api_resp::RespData::None,
                        echo: "".to_string(),
                    });
                let auth = if let Some(TypedHeader(auth)) = authorization {
                    Some(auth.0)
                } else {
                    None
                };
                action_sender
                    .send(crate::Action::AddBot {
                        bot_id: x_self_id.0.clone(),
                        api_sender: sender,
                        action_sender: action_sender.clone(),
                        auth: auth,
                        api_resp_watcher: api_resp_watcher,
                    })
                    .await
                    .unwrap();
                event!(
                    Level::INFO,
                    "{} Client {} is connected. The client type is {}",
                    user_agent.to_string().bright_yellow(),
                    x_self_id.0.red(),
                    x_client_role.0.bright_cyan()
                );
                handle_socket(
                    socket,
                    receiver,
                    event_sender,
                    action_sender,
                    apiresp_watch_sender,
                    x_self_id.0,
                )
                .await;
            } else {
                event!(Level::WARN, "Client headers wrong.");
                socket.close().await.unwrap();
            }
        };
    let app = route("/ws", ws(handle_socket));
    event!(Level::INFO, "Serving at -> ws://{}:{}/ws", host, port);
    axum::Server::bind(&std::net::SocketAddr::from((host, port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn stream_recv(
    stream: futures_util::stream::SplitStream<axum::ws::WebSocket>,
    event_sender: &EventSender,
    action_sender: &ActionSender,
    apiresp_watch_sender: &watch::Sender<crate::api_resp::ApiResp>,
    bot_id: String,
) -> Option<futures_util::stream::SplitStream<axum::ws::WebSocket>> {
    let (msg, next_stream) = stream.into_future().await;
    if let Some(msg) = msg {
        use crate::event::RecvItem;
        if let Ok(msg) = msg {
            let data: serde_json::Result<RecvItem> = serde_json::from_str(msg.to_str().unwrap());
            match data {
                Ok(data) => match data {
                    RecvItem::Event(event) => {
                        event_sender.send(event).unwrap();
                    }
                    RecvItem::ApiResp(api_resp) => {
                        apiresp_watch_sender.send(api_resp).unwrap();
                    }
                },
                Err(e) => {
                    tracing::event!(
                        tracing::Level::ERROR,
                        "Serialize Msg failed! Msg:{:?}\nError:{}",
                        msg.to_str().unwrap(),
                        e
                    );
                    std::process::exit(101);
                }
            }
        } else {
            tracing::event!(
                tracing::Level::WARN,
                "Bot [{}] {}",
                bot_id.to_string().red(),
                "disconnect."
            );
            action_sender
                .send(crate::Action::RemoveBot { bot_id: bot_id })
                .await
                .unwrap();
            return None;
        }
    }
    Some(next_stream)
}

async fn handle_socket(
    socket: WebSocket,
    mut api_receiver: mpsc::Receiver<crate::ApiChannelItem>,
    event_sender: EventSender,
    action_sender: ActionSender,
    apiresp_watch_sender: watch::Sender<crate::api_resp::ApiResp>,
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
                    sink.send(axum::ws::Message::text(json_string))
                        .await
                        .unwrap();
                }
                // temp Matcher event
                crate::ApiChannelItem::MessageEvent(_) => {
                    use colored::*;
                    tracing::event!(
                        tracing::Level::WARN,
                        "{}",
                        "WedSocket接受端接收到错误Event消息".bright_red()
                    );
                }
                // temp Matcher Timeout
                crate::ApiChannelItem::TimeOut => {
                    use colored::*;
                    tracing::event!(
                        tracing::Level::WARN,
                        "{}",
                        "WedSocket接受端接收到错误TimeOut消息".bright_red()
                    );
                } // 忽视 event 该 receiver 永不应该收到 event
            }
        }
    };
    tokio::spawn(income);
    tokio::spawn(outcome);
}
