mod drivers;
mod log;

pub async fn run() {
    log::init();
    crate::drivers::axum::run().await;
}
