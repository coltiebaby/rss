use tungstenite::{accept, client, Message};
use std::net::TcpStream;

#[tokio::main]
async fn main() {
    let lc = lcu::client::Client::new().unwrap();
    let req = lc.wss().unwrap();
    let connector = lcu::connector::Connector::builder().insecure(true).connect(req).unwrap();

    let (rx, handle) = connector.spawn().unwrap();

    while let Ok(msg) = rx.recv_async().await {
        println!("recv msg: {msg:?}");
    }
}
