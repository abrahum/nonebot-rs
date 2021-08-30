use crate::bot::{Action, Bot, ChannelItem};
use crate::Nonebot;
use axum::{
    extract::TypedHeader,
    prelude::*,
    ws::{ws, WebSocket},
};
use colored::*;
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};
use tracing::{event, Level};

mod xheaders;

pub async fn run(nb_arc: Arc<Mutex<Nonebot>>) {
    let host: std::net::Ipv4Addr;
    let port: u16;
    {
        let nb = nb_arc.lock().unwrap();
        host = nb.config.global.host;
        port = nb.config.global.port;
    }
    let (broadcaster, _) = broadcast::channel::<Action>(32);
    let handle_socket =
        |socket: WebSocket,
         x_self_id: Option<TypedHeader<xheaders::XSelfId>>,
         x_client_role: Option<TypedHeader<xheaders::XClientRole>>,
         user_agent: Option<TypedHeader<headers::UserAgent>>,
         authorization: Option<TypedHeader<xheaders::Authorization>>| async move {
            event!(Level::INFO, "A client is connectted.");
            if let (
                Some(TypedHeader(user_agent)),
                Some(TypedHeader(x_self_id)),
                Some(TypedHeader(x_client_role)),
            ) = (user_agent, x_self_id, x_client_role)
            {
                let (sender, receiver) = mpsc::channel(32);
                let broadcaster = broadcaster.clone();
                let auth = if let Some(TypedHeader(auth)) = authorization {
                    Some(auth.0)
                } else {
                    None
                };
                {
                    let mut nb = nb_arc.lock().unwrap();
                    (*nb).add_bot(x_self_id.0, sender.clone()); // 在 nb 建立 bot 状态管理器
                }
                let bot = Bot::new(
                    x_self_id.0,
                    auth,
                    sender,
                    broadcaster.subscribe(),
                    nb_arc.clone(),
                )
                .unwrap();
                event!(
                    Level::INFO,
                    "{} Client {} is connectted. The client type is {}",
                    user_agent.to_string().bright_yellow(),
                    x_self_id.0.to_string().red(),
                    x_client_role.0.bright_cyan()
                );
                handle_socket(bot, socket, receiver, broadcaster).await;
            }
        };
    let app = route("/ws", ws(handle_socket));
    event!(Level::INFO, "Serveing at -> ws://{}:{}/ws", host, port);
    axum::Server::bind(&std::net::SocketAddr::from((host, port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn stream_recv(
    stream: futures_util::stream::SplitStream<axum::ws::WebSocket>,
    mut bot: Bot,
) -> (futures_util::stream::SplitStream<axum::ws::WebSocket>, Bot) {
    let (msg, next_stream) = stream.into_future().await;
    if let Some(msg) = msg {
        if let Ok(msg) = msg {
            bot.handle_recv(msg.to_str().unwrap().to_string()).await;
        }
    }
    (next_stream, bot)
}

async fn handle_socket(
    mut bot: Bot,
    socket: WebSocket,
    mut receiver: mpsc::Receiver<ChannelItem>,
    broadcaster: broadcast::Sender<Action>,
) {
    // 将 websocket 接收流与发送流分离
    let (mut sink, mut stream) = socket.split();
    // 接收消息
    let income = async move {
        loop {
            let rdata = stream_recv(stream, bot).await;
            stream = rdata.0;
            bot = rdata.1;
        }
    };
    // 发送消息
    let outcome = async move {
        while let Some(data) = receiver.recv().await {
            match data {
                ChannelItem::Api(data) => {
                    let json_string = serde_json::to_string(&data).unwrap();
                    sink.send(axum::ws::Message::text(json_string))
                        .await
                        .unwrap();
                }
                ChannelItem::Action(action) => {
                    broadcaster.send(action).unwrap();
                }
                ChannelItem::MessageEvent(_) => {
                    use colored::*;
                    tracing::event!(
                        tracing::Level::WARN,
                        "{}",
                        "WedSocket接受端接收到错误Event消息".bright_red()
                    );
                }
                ChannelItem::TimeOut => {
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
