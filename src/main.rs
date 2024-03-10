use league_client::client;
use league_client::connector::subscribe;
use league_client::core::EventType;

use rand::prelude::SliceRandom;

#[tokio::main]
async fn main() {
    let c = client::Client::builder().unwrap().insecure(true).build().unwrap();
    let connected = c.connect_to_socket().await.unwrap();
    let speaker = subscribe(connected).await;

    let msg = (5, "OnJsonApiEvent");
    let msg = serde_json::to_string(&msg).unwrap();

    speaker.send(msg).await.expect("should have sent a message");

    while let Ok(incoming) = speaker.reader.recv_async().await {
        let message = incoming.into_message();

        if &message.uri != "/lol-champ-select/v1/skin-carousel-skins" {
            continue;
        }

        if message.event_type != EventType::Create {
            continue;
        }

        let skins: Vec<rss::CarouselSkin> = serde_json::from_value(message.data).unwrap();

        if skins.is_empty() {
            continue;
        }

        let ids: Vec<i64> = skins.iter().filter(|x| x.unlocked && !x.disabled).map(|x| x.id).collect();
        let selected_skin_id = ids.choose(&mut rand::thread_rng()).unwrap().clone();

        let select = rss::Select {
            selected_skin_id,
            spell_1_id: 14,
            spell_2_id: 4,
            ward_skin_id: 1,
        };

        let data = serde_json::to_string(&select).unwrap();

        let http_client = c.http_client();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        let address = format!("https://{}/{}", &c.addr, "lol-champ-select/v1/session/my-selection");
        let req = http_client.patch(&address).body(data);

        let resp = req
            .send()
            .await
            .unwrap();
    }
}
