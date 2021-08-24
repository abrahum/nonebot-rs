use crate::api::Apis;
use crate::bot::Bot;
use crate::Nonebot;
use axum::{
    extract::TypedHeader,
    prelude::*,
    ws::{ws, WebSocket},
};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
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
                let (sender, receiver) = mpsc::channel(1);
                let auth = if let Some(TypedHeader(auth)) = authorization {
                    Some(auth.0)
                } else {
                    None
                };
                {
                    let mut nb = nb_arc.lock().unwrap();
                    if let Some(bot_in_nb) = nb.bots.get_mut(&x_self_id.0.to_string()) {
                        bot_in_nb.sender = Some(sender.clone());
                    }
                }
                let bot = Bot::new(x_self_id.0, auth, sender, nb_arc.clone());
                event!(
                    Level::INFO,
                    "{} Client {} is connectted. The client type is {}",
                    user_agent,
                    x_self_id.0,
                    x_client_role.0
                );
                handle_socket(bot, socket, receiver).await;
            }
        };
    let app = route("/ws", ws(handle_socket));
    event!(Level::INFO, "Serveing at -> ws://{}:{}/ws", host, port);
    axum::Server::bind(&std::net::SocketAddr::from((host, port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_socket(bot: Bot, mut socket: WebSocket, mut receiver: mpsc::Receiver<Apis>) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                bot.handle_event(msg.to_str().unwrap().to_string()).await;
            }
        }
        if let Some(data) = receiver.recv().await {
            if let crate::api::Apis::None = data {
            } else {
                let json_string = serde_json::to_string(&data).unwrap();
                socket
                    .send(axum::ws::Message::text(json_string))
                    .await
                    .unwrap();
            }
        }
    }
}
