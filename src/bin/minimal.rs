use nonebot_rs;

fn main() {
    let mut nb = nonebot_rs::Nonebot::new();
    nb.matchers
        .add_message_matcher(nonebot_rs::builtin::echo::echo());
    nb.run()
}
