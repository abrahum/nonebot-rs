use super::utils::handler_web_socket;
use crate::{ActionSender, EventSender};
use tokio::net::TcpStream;
use tracing::{event, Level};

use tokio_tungstenite::{client_async, tungstenite::handshake::client::Request};

pub async fn run(
    urls: Vec<String>,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    for url in urls {
        tokio::spawn(single_socket(
            url,
            event_sender.clone(),
            action_sender.clone(),
            access_token.clone(),
        ));
    }
}

pub async fn single_socket(
    url: String,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    let req = Request::builder().uri(&url).body(()).unwrap();

    event!(Level::INFO, "Connecting to {}", url);

    let tcp_stream = TcpStream::connect(url).await.expect("Tcp connect error");
    let ws_stream = client_async(req, tcp_stream).await;
    match ws_stream {
        Ok((stream, _)) => handler_web_socket(stream, event_sender, action_sender).await,
        Err(_) => {}
    }
}
