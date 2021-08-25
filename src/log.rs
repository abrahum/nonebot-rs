pub fn init(debug: bool) {
    if debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
}
