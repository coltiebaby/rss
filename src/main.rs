#[tokio::main]
async fn main() {
    env_logger::init();

    loop {
        if let Err(e) = rss::run().await {
            log::error!("run stopped: {e:?}");
        }
    }
}
