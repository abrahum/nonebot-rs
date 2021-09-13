/// Bot Status
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod bot_status;
/// 内建 echo Matcher
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod echo;
#[doc(hidden)]
pub mod macros;
/// 内建 PreMatcher 函数
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod prematchers;
/// rcnb！！！
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod rcnb;
/// 内建 rules
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod rules;

use tracing::{event, Level};

#[doc(hidden)]
pub fn resp_logger(resp: &crate::api_resp::ApiResp) {
    if &resp.status == "ok" {
        event!(Level::DEBUG, "{} success", resp.echo);
    } else {
        event!(Level::INFO, "{} failed", resp.echo);
    }
}
