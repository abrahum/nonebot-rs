use nonebot_rs::{Job, Message};

pub fn clock(nb: &nonebot_rs::Nonebot) -> Job {
    let bot_getter = nb.bot_getter.clone();
    Job::new("1 * * * * *", move |_, _| {
        let bots = bot_getter.borrow().clone();
        for (_, bot) in bots {
            let bot = bot.clone();
            tokio::spawn(get_and_send_group_count(bot));
        }
    })
    .unwrap()
}

// Just for test
async fn get_and_send_group_count(bot: nonebot_rs::Bot) {
    let data = bot.call_api_resp(nonebot_rs::Api::get_group_list()).await;
    if let Some(resp) = data {
        if let nonebot_rs::RespData::GroupList(glist) = &resp.data {
            for superuser in &bot.config.superusers {
                bot.send_private_msg(
                    superuser.parse().unwrap(),
                    vec![Message::text(&format!("{}", glist.len()))],
                )
                .await;
            }
        }
    }
}
