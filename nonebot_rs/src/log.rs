pub use colored;
pub use tracing::{event, Level};

pub fn init(debug: bool, trace: Option<bool>) {
    if debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    if let Some(b) = trace {
        if b {
            std::env::set_var("RUST_LOG", "trace");
        }
    }
    tracing_subscriber::fmt::init();
}
