use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub async fn handler_web_socket(socket: WebSocketStream<TcpStream>) {}
