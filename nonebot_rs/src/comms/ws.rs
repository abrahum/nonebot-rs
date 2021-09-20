use super::utils::handler_web_socket;
use crate::{event::Event, matcher::prelude::SelfId, ActionSender, EventSender};
use async_recursion::async_recursion;
use colored::*;
use futures_util::StreamExt;
use tokio::{
    net::TcpStream,
    sync::{mpsc, watch},
};
use tracing::{event, Level};

use tokio_tungstenite::{client_async, tungstenite::handshake::client::Request};

#[async_recursion]
pub async fn run(
    url: String,
    bot_id: String,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    single_socket(
        &url,
        &bot_id,
        event_sender.clone(),
        action_sender.clone(),
        access_token.clone(),
    )
    .await;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    run(url, bot_id, event_sender, action_sender, access_token).await;
}

pub async fn single_socket(
    url: &str,
    bot_id: &str,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) -> () {
    let req = Request::builder()
        .uri(url)
        .header("Authorization", access_token.get(bot_id))
        .body(())
        .unwrap();
    let host = req.uri().host().unwrap().to_string();
    let port = req
        .uri()
        .port_u16()
        .or_else(|| match req.uri().scheme_str() {
            Some("wss") => Some(443),
            Some("ws") => Some(80),
            _ => None,
        })
        .ok_or("unable to get port")
        .unwrap();
    let addr = format!("{}:{}", host, port);

    event!(Level::INFO, "Connecting to {}", url);

    let tcp_stream = match TcpStream::connect(addr).await {
        Ok(s) => s,
        Err(_) => return (),
    };

    // build channel
    let (sender, receiver) = mpsc::channel(32);
    let (apiresp_watch_sender, api_resp_watcher) = watch::channel(crate::api_resp::ApiResp {
        status: "init".to_string(),
        retcode: 0,
        data: crate::api_resp::RespData::None,
        echo: "".to_string(),
    });

    let ws_stream = client_async(req, tcp_stream).await;
    match ws_stream {
        Ok((mut stream, _)) => {
            // let headers = resp.headers();
            // println!("{:?}", headers);
            if let Some(data) = stream.next().await {
                match data {
                    Ok(msg) => {
                        let msg = msg.to_text().unwrap();
                        let event: Event = serde_json::from_str(msg).unwrap();
                        let bot_id = event.get_self_id();

                        event!(Level::INFO, "Connectted to Bot {} Server", bot_id.red());

                        // add bot to Nonebot
                        action_sender
                            .send(crate::Action::AddBot {
                                bot_id: bot_id.clone(),
                                api_sender: sender,
                                action_sender: action_sender.clone(),
                                api_resp_watcher: api_resp_watcher,
                            })
                            .await
                            .unwrap();

                        // handle WebSocketStream
                        handler_web_socket(
                            stream,
                            event_sender,
                            action_sender,
                            apiresp_watch_sender,
                            receiver,
                            bot_id,
                        )
                        .await;
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
}
