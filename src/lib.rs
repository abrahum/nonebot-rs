mod axum;
mod event;
mod log;
mod message;

#[macro_use]
extern crate lazy_static;

pub async fn run() {
    log::init();
    crate::axum::run().await;
}
