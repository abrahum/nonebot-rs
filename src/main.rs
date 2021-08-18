mod drivers;
mod log;

#[tokio::main]
async fn main() {
    log::init();
    drivers::axum::run().await;
}
