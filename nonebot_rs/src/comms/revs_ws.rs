use super::utils::handler_web_socket;
use crate::{ActionSender, EventSender};
use tokio::net::{TcpListener, TcpStream};
use tracing::{event, Level};

pub async fn run(
    host: std::net::Ipv4Addr,
    port: u16,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    let try_socket = TcpListener::bind(std::net::SocketAddrV4::new(host, port)).await;
    let listener = try_socket.expect("Socket Bind fail");

    event!(Level::INFO, "Serveing at -> ws://{}:{}/ws", host, port);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(
            stream,
            event_sender.clone(),
            action_sender.clone(),
            access_token.clone(),
        ));
    }
}

async fn accept_connection(
    stream: TcpStream,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    event!(Level::DEBUG, "Get Tcp stream from {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    handler_web_socket(ws_stream).await;
}
