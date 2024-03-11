use league_client::client;
use league_client::connector::subscribe;
use league_client::core::EventType;

use rand::prelude::SliceRandom;

#[tokio::main]
async fn main() {
    let c = client::Client::builder().unwrap().insecure(true).build().unwrap();
    let connected = c.connect_to_socket().await.unwrap();
    let speaker = subscribe(connected).await;

    let http_client = c.http_client();
    let resp = http_client
        .get(format!("https://{}/{}", &c.addr, "lol-summoner/v1/current-summoner"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{resp}");
    let session: rss::Session = serde_json::from_str(&resp).unwrap();

    let msg = (5, "OnJsonApiEvent");
    let msg = serde_json::to_string(&msg).unwrap();

    speaker.send(msg).await.expect("should have sent a message");

    let mut selection = rss::Selection::default();

    while let Ok(incoming) = speaker.reader.recv_async().await {
        let message = incoming.into_message();

        if &message.uri == "/lol-champ-select/v1/session" {
            selection = serde_json::from_value(message.data).unwrap();
            continue;
        }

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

        let mut spell1id = 0;
        let mut spell2id = 0;
        let mut ward_skin_id = 0;
        for player in selection.my_team.iter() {
            if player.summoner_id != session.summoner_id {
                continue;
            }

            spell1id = player.spell1id;
            spell2id = player.spell2id;
            ward_skin_id = player.ward_skin_id;
        }

        let ids: Vec<i64> = skins.iter().filter(|x| x.unlocked && !x.disabled).map(|x| x.id).collect();
        let selected_skin_id = ids.choose(&mut rand::thread_rng()).unwrap().clone();

        let select = rss::Select {
            selected_skin_id,
            spell1id,
            spell2id,
            ward_skin_id,
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
