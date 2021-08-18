use axum::{
    prelude::*,
    ws::{ws, WebSocket},
};
use tracing::{event, Level};

pub async fn run() {
    let host = std::net::IpAddr::from([127, 0, 0, 1]);
    let port = 8088u16;
    let app = route("/ws", ws(handle_socket));
    event!(Level::INFO, "Serveing at -> ws://{}:{}/ws", host, port);
    axum::Server::bind(&std::net::SocketAddr::from((host, port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_socket(mut socket: WebSocket) {
    event!(Level::INFO, "A client is connectted.");
    while let Some(msg) = socket.recv().await {
        let msg = msg.unwrap();
        event!(
            Level::INFO,
            "Recived message: {}",
            msg.to_str().unwrap().replace("\n", "")
        );
        socket.send(msg).await.unwrap();
    }
}
