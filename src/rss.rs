use anyhow::Result;
use rand::prelude::SliceRandom;
use std::sync::Arc;
use tokio::sync::RwLock;

use league_client::client;
use league_client::connector::subscribe;
// use league_client::core::EventType;

pub type State = Arc<RwLock<Locker>>;

#[derive(Default)]
pub struct Locker {
    client: reqwest::Client,
    addr: String,
    session: super::core::Session,
    select: super::core::Select,
}

impl From<Locker> for State {
    fn from(locker: Locker) -> State {
        Arc::new(RwLock::new(locker))
    }
}

fn tries(mut counter: u64) -> tokio::time::Duration {
    log::trace!("attempt {counter}");
    if counter > 3 {
        counter = 3;
    }

    tokio::time::Duration::from_secs(counter * 10)
}

impl Locker {
    pub async fn new(client: reqwest::Client, addr: String) -> Result<Self> {
        let req = format!("https://{}/{}", &addr, "lol-summoner/v1/current-summoner");

        let session: super::core::Session;

        let mut counter: u64 = 1;
        loop {
            let resp = client.get(&req).send().await?.text().await?;
            if let Ok(s) = serde_json::from_str::<super::core::Session>(&resp) {
                session = s;
                break;
            }

            tokio::time::sleep(tries(counter)).await;
            counter += 1;
        }

        Ok(Self {
            client,
            session,
            addr,
            ..Default::default()
        })
    }

    fn update_select(&mut self, other: super::core::Select) {
        self.select = other;
    }

    pub fn summoner_id(&self) -> i64 {
        self.session.summoner_id
    }

    pub fn select(&self) -> super::core::Select {
        let select = self.select.clone();

        select
    }

    pub async fn pick_this_skin(&self, select: super::core::Select) -> Result<()> {
        let address = format!(
            "https://{}/{}",
            &self.addr, "lol-champ-select/v1/session/my-selection"
        );

        let data = serde_json::to_string(&select).unwrap();
        self.client.patch(&address).body(data).send().await?;

        Ok(())
    }
}

pub async fn current_spells_and_ward_skin(
    state: State,
    rx: flume::Receiver<league_client::core::Message>,
) -> Result<()> {
    loop {
        let msg = rx.recv_async().await?;
        let summoner_id = state.read().await.summoner_id();

        let selection: super::core::Selection = serde_json::from_value(msg.data)?;
        for player in selection.my_team.iter() {
            if player.summoner_id != summoner_id {
                continue;
            }

            let select = super::core::Select::from(player.clone());
            state.write().await.update_select(select);
        }
    }
}

pub async fn select_skin(
    state: State,
    skins: Vec<super::core::CarouselSkin>,
) -> Result<super::core::Select> {
    if skins.is_empty() {
        anyhow::bail!("no skins");
    }

    let ids: Vec<i64> = skins
        .iter()
        .filter(|x| x.unlocked && !x.disabled)
        .map(|x| x.id)
        .collect();
    let selected_skin_id = ids
        .choose(&mut rand::thread_rng())
        .ok_or(anyhow::anyhow!("could not make a random number"))?
        .clone();

    let mut select = state.read().await.select();
    select.selected_skin_id = selected_skin_id;

    Ok(select)
}

struct Consumer {
    state: State,
    speaker: league_client::Speaker,
    session_tx: flume::Sender<league_client::core::Message>,
}

impl Consumer {
    pub async fn spawn(&mut self) -> Result<()> {
        let msg = (5, "OnJsonApiEvent");
        let msg = serde_json::to_string(&msg)?;
        self.speaker.send(msg).await?;

        while let Ok(incoming) = self.speaker.reader.recv_async().await {
            let message = incoming.into_message();

            if &message.uri == "/lol-champ-select/v1/session" {
                if let Err(e) = self.session_tx.send(message) {
                    log::warn!("failed to send {e}");
                }

                continue;
            }

            // Skip unwanted stuff.
            if &message.uri != "/lol-champ-select/v1/skin-carousel-skins" {
                continue;
            }

            let skins = serde_json::from_value::<Vec<crate::CarouselSkin>>(message.data);

            if skins.is_err() {
                continue;
            }

            let skins = skins.unwrap();

            if skins.is_empty() {
                continue;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

            let skin = match select_skin(self.state.clone(), skins).await {
                Ok(skin) => skin,
                Err(e) => {
                    log::error!("failed to select the skin: {:?}", e);
                    continue;
                }
            };

            if let Err(e) = self.state.read().await.pick_this_skin(skin).await {
                log::error!("failed to pick skin: {e}");
                continue;
            }
        }

        anyhow::bail!("speaker was dropped");
    }
}

pub async fn run() -> Result<()> {
    let builder: client::ClientBuilder;

    let mut counter: u64 = 1;
    loop {
        match client::Client::builder() {
            Err(_) => {
                log::info!("waiting for client to be active");
                tokio::time::sleep(tries(counter)).await;
            }
            Ok(b) => {
                builder = b;
                break;
            }
        }

        counter += 1;
    }

    let c = builder.insecure(true).build().unwrap();

    let connected = c.connect_to_socket().await.unwrap();
    let speaker = subscribe(connected).await;
    let http_client = c.http_client();

    let state = crate::Locker::new(http_client, c.addr.clone())
        .await
        .expect("failed to create the http client");
    let state: State = state.into();

    let (session_tx, session_rx) = flume::unbounded();
    let _handle_1 = tokio::task::spawn(crate::current_spells_and_ward_skin(
        state.clone(),
        session_rx,
    ));

    let mut consumer = Consumer {
        state,
        speaker,
        session_tx,
    };

    log::info!("rolling the dice");
    consumer.spawn().await?;

    anyhow::bail!("run stopped early");
}
