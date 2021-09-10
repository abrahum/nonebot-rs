use nonebot_rs::{Job, Message};

#[allow(dead_code)]
pub fn clock(nb: &nonebot_rs::Nonebot) -> Job {
    let bot_getter = nb.bot_getter.clone();
    let _some_config = nb
        .config
        .get_job_config("job_name")
        .and_then(|job_config| job_config.get("config_item"));
    Job::new("1 * * * * *", move |_, _| {
        let bots = bot_getter.borrow().clone();
        for (_, bot) in bots {
            let bot = bot.clone();
            tokio::spawn(send_a_msg(bot));
        }
    })
    .unwrap()
}

// Just for test
#[allow(dead_code)]
async fn send_a_msg(bot: nonebot_rs::Bot) {
    for superuser in &bot.config.superusers {
        bot.send_private_msg(
            superuser,
            vec![Message::text("One minute passed.".to_string())],
        )
        .await;
    }
}
