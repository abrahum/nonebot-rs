use nonebot_rs::Job;

pub fn clock(nb: &nonebot_rs::Nonebot) -> Job {
    let bot_getter = nb.bot_getter.clone();
    Job::new("1 * * * * *", move |_, _| {
        let bot = bot_getter.borrow().clone();
        println!("{:#?}", bot);
    })
    .unwrap()
}
