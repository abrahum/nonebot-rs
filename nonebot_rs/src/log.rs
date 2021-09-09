pub use colored;
#[cfg(feature = "matcher")]
use colored::*;
pub use tracing::{event, Level};

pub fn init(debug: bool, trace: Option<bool>) {
    if debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    if let Some(b) = trace {
        if b {
            std::env::set_var("RUST_LOG", "nonebot_rs=trace");
        }
    }
    tracing_subscriber::fmt::init();
}

#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub fn log_load_matchers(matchers: &crate::Matchers) {
    log_matcherb(&matchers.message);
    log_matcherb(&matchers.notice);
    log_matcherb(&matchers.request);
    log_matcherb(&matchers.meta);
}

#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
fn log_matcherb<E>(matcherb: &crate::MatchersBTreeMap<E>)
where
    E: Clone,
{
    if matcherb.is_empty() {
        return;
    }
    for (_, matcherh) in matcherb {
        for (name, _) in matcherh {
            event!(Level::INFO, "Matcher {} is Loaded", name.blue());
        }
    }
}
