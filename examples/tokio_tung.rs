#[tokio::main]
async fn main() {
    let lc = lcu::client::Client::new().unwrap();
    let req = lc.wss().unwrap();

    let builder = lcu::connector::Connector::builder();
    let c = builder.insecure(true).build();

    let _ = c.connect(req);
}
