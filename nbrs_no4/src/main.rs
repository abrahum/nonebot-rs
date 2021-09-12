use nbrs_matcher_r6s::r6s;
use nonebot_rs;

mod clock;

fn main() {
    let mut nb = nonebot_rs::Nonebot::new();
    let config = nb.config.clone();
    let mut matchers = nonebot_rs::Matchers::new_empty();
    matchers
        .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb())
        .add_message_matcher(nonebot_rs::builtin::echo::echo2())
        .add_message_matcher(nonebot_rs::builtin::bot_status::bot_status(
            config.get_matcher_config("bot_status"),
        ));
    nb.add_plugin("Matchers", std::sync::Arc::new(matchers));

    // let lua_config = nb.config.lua.clone();
    // let lua = nonebot_rs::lua::LuaPlugin::new(if let Some(config) = lua_config {
    //     config
    // } else {
    //     std::collections::HashMap::new()
    // });
    // nb.add_plugin("Lua", std::sync::Arc::new(lua));

    // nb.matchers
    //     .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb())
    //     .add_message_matcher(nonebot_rs::builtin::echo::echo2())
    //     .add_message_matcher(nonebot_rs::builtin::bot_status::bot_status(
    //         config.get_matcher_config("bot_status"),
    //     ))
    //     .add_message_matchers(r6s());
    // nb.scheduler.add(clock::clock(&nb)).unwrap();
    nb.run()
}
