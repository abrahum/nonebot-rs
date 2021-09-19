use super::utils::handler_web_socket;
use crate::{ActionSender, EventSender};
use tokio::net::TcpStream;
use tracing::{event, Level};

use tokio_tungstenite::{client_async, tungstenite::handshake::client::Request};

pub async fn run(
    url: &str,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    let req = Request::builder().uri(url).body(()).unwrap();

    event!(Level::INFO, "Connecting to {}", url);

    let tcp_stream = TcpStream::connect(url).await.expect("Tcp connect error");
    let ws_stream = client_async(req, tcp_stream).await;
    match ws_stream {
        Ok((stream, _)) => handler_web_socket(stream).await,
        Err(_) => {}
    }
}
