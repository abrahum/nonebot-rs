pub fn init() {
    std::env::set_var("RUST_LOG", "nonebot_rs=trace");
    tracing_subscriber::fmt::init();
}
