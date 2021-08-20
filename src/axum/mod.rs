use crate::event::{Events, MessageEvent};
use crate::Nonebot;
use axum::{
    extract::TypedHeader,
    prelude::*,
    ws::{ws, WebSocket},
};
use std::sync::{Arc, Mutex};
use tracing::{event, Level};

mod xheaders;

pub async fn run(nbarc: Arc<Mutex<Nonebot>>) {
    let host = std::net::IpAddr::from([127, 0, 0, 1]);
    let port = 8088u16;
    let handle_socket =
        |mut socket: WebSocket,
         x_self_id: Option<TypedHeader<xheaders::XSelfId>>,
         x_client_role: Option<TypedHeader<xheaders::XClientRole>>,
         user_agent: Option<TypedHeader<headers::UserAgent>>,
         authorization: Option<TypedHeader<xheaders::Authorization>>| async move {
            let nbarcc = Arc::clone(&nbarc);
            event!(Level::INFO, "A client is connectted.");
            if let (
                Some(TypedHeader(user_agent)),
                Some(TypedHeader(x_self_id)),
                Some(TypedHeader(x_client_role)),
            ) = (user_agent, x_self_id, x_client_role)
            {
                let mut nb = nbarcc.lock().unwrap();
                (*nb)
                    .bots
                    .insert(x_self_id.0.to_string(), crate::bot::Bot::new(x_self_id.0));
                event!(
                    Level::INFO,
                    "{} Client {} is connectted. The client type is {}",
                    user_agent,
                    x_self_id.0,
                    x_client_role.0
                );
            }
            while let Some(msg) = socket.recv().await {
                if let Ok(msg) = msg {
                    let msg: &str = msg.to_str().unwrap();
                    handle_event(&nbarcc, msg);
                }
            }
        };
    let app = route("/ws", ws(handle_socket));
    event!(Level::INFO, "Serveing at -> ws://{}:{}/ws", host, port);
    axum::Server::bind(&std::net::SocketAddr::from((host, port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn handle_event(nb: &Arc<Mutex<Nonebot>>, msg: &str) {
    let ievent: Events = serde_json::from_str(msg).unwrap();
    match ievent {
        Events::Message(e) => {
            let msg = match e {
                MessageEvent::Private(p) => p.raw_message,
                MessageEvent::Group(g) => g.raw_message,
            };
            event!(Level::INFO, "Recive message: {}", msg);
        }
        Events::Notice(_) => {}
        Events::Request(_) => {}
        Events::Meta(e) => {
            if &e.meta_event_type == "heartbeat" {
                event!(Level::INFO, "Recive HeartBeat.")
            }
        }
    }
}
