mod axum;
mod event;
mod log;
mod message;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    log::init();
    axum::run().await;
}
