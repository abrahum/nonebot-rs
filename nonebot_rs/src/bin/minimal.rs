use nonebot_rs;

fn main() {
    let mut nb = nonebot_rs::Nonebot::new();
    #[cfg(feature = "matcher")]
    let mut matchers = nonebot_rs::Matchers::new_empty();
    matchers
        .add_message_matcher(nonebot_rs::builtin::echo::echo2())
        .add_message_matcher(nonebot_rs::builtin::echo::echo())
        .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb());
    nb.add_plugin("Matchers", std::sync::Arc::new(matchers));
    nb.run()
}
