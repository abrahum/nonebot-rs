pub mod revs_ws;
pub mod utils;
pub mod ws;

pub async fn strat_comms(nb: &crate::Nonebot) {
    let access_token = nb.config.gen_access_token();

    if let Some(ws_server_config) = &nb.config.ws_server {
        tokio::spawn(revs_ws::run(
            ws_server_config.host,
            ws_server_config.port,
            nb.event_sender.clone(),
            nb.action_sender.clone(),
            access_token.clone(),
        ));
    }

    if let Some(bots) = &nb.config.bots {
        for (bot_id, bot_config) in bots {
            if !bot_config.ws_server.is_empty() {
                tokio::spawn(ws::run(
                    bot_config.ws_server.clone(),
                    bot_id.clone(),
                    nb.event_sender.clone(),
                    nb.action_sender.clone(),
                    access_token.clone(),
                ));
            }
        }
    }
}
