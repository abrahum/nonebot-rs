use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn clock(nb: &nonebot_rs::Nonebot) {
    let bot_getter = nb.bot_getter.clone();
    let mut sched = JobScheduler::new();

    sched
        .add(
            Job::new("1 * * * * *", move |_, _| {
                let bot = bot_getter.borrow().clone();
                println!("{:#?}", bot);
            })
            .unwrap(),
        )
        .unwrap();

    sched.start().await.unwrap();
}
