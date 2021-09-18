// use nbrs_matcher_r6s::r6s;
use nonebot_rs;

mod clock;

fn main() {
    let mut nb = nonebot_rs::Nonebot::new();

    let mut matchers = nonebot_rs::Matchers::new_empty();
    matchers
        .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb())
        .add_message_matcher(nonebot_rs::builtin::echo::echo2());
    nb.add_plugin(matchers);

    let lua = nbrs_lua::LuaPlugin::new();
    nb.add_plugin(lua);

    let mut scheduler = nonebot_rs::Scheduler::new();
    scheduler.add_job(clock::clock(&nb));
    nb.add_plugin(scheduler);

    nb.run()
}
