use league_client::client;
use league_client::connector::subscribe;
use league_client::core::EventType;

use rss::*;

#[tokio::main]
async fn main() {
    let builder: client::ClientBuilder;

    loop {
        match client::Client::builder() {
            Err(_) => {
                tracing::info!("waiting for client to be active");
                let ten_secs = std::time::Duration::from_secs(10);
                std::thread::sleep(ten_secs);
            }
            Ok(b) => {
                builder = b;
                break;
            }
        }
    }

    println!("we're connected");

    let c = builder.insecure(true).build().unwrap();

    let connected = c.connect_to_socket().await.unwrap();
    let speaker = subscribe(connected).await;
    let http_client = c.http_client();

    let state = rss::Locker::new(http_client, c.addr.clone())
        .await
        .expect("failed to create the http client");
    let state: State = state.into();

    let msg = (5, "OnJsonApiEvent");
    let msg = serde_json::to_string(&msg).unwrap();
    speaker.send(msg).await.expect("should have sent a message");

    let (session_tx, session_rx) = flume::unbounded();
    let _handle = tokio::task::spawn(rss::current_spells_and_ward_skin(state.clone(), session_rx));

    let mut counter = 0;
    while let Ok(incoming) = speaker.reader.recv_async().await {
        let message = incoming.into_message();

        if &message.uri == "/lol-champ-select/v1/session" {
            if let Err(e) = session_tx.send(message) {
                tracing::warn!("failed to send {e}");
            }

            continue;
        }

        // Skip unwanted stuff.
        if &message.uri != "/lol-champ-select/v1/skin-carousel-skins" {
            continue;
        }

        let skins = serde_json::from_value::<Vec<rss::CarouselSkin>>(message.data);

        if skins.is_err() {
            continue;
        }

        let skins = skins.unwrap();

        if skins.is_empty() {
            continue;
        }

        let _ = tokio::time::sleep(tokio::time::Duration::from_secs(3));

        let skin = select_skin(state.clone(), skins).await.unwrap();
        if let Err(e) = state.read().await.pick_this_skin(skin).await {
            println!("failed to pick skin {e}");
            continue;
        }
    }
}
