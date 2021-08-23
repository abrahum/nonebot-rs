pub fn init(debug: bool) {
    if debug {
        std::env::set_var("RUST_LOG", "nonebot_rs=trace");
    } else {
        std::env::set_var("RUST_LOG", "nonebot_rs=info")
    }
    tracing_subscriber::fmt::init();
}
