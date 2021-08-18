pub fn init() {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();
}
