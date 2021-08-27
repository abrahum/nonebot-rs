use nonebot_rs;

fn main() {
    let mut nb = nonebot_rs::Nonebot::new();
    nb.matchers
        .add_message_matcher(nonebot_rs::builtin::echo::echo2())
        .add_message_matcher(nonebot_rs::builtin::echo::echo())
        .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb());
    println!("{:?}", nb.matchers);
    nb.run()
}
