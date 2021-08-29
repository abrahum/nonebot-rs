use nbrs_matcher_r6s::r6s;
use nonebot_rs;

fn main() {
    let mut nb = nonebot_rs::Nonebot::new();
    nb.matchers
        .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb())
        .add_message_matchers(r6s());
    nb.run()
}
