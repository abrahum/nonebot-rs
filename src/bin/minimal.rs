use nonebot_rs;

fn main() {
    let nb = nonebot_rs::Nonebot::new(nonebot_rs::Matchers::new(
        Some(vec![nonebot_rs::butin::echo_::echo()]),
        None,
        None,
        None,
    ));
    nb.run()
}
