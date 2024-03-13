use anyhow::Result;
use rand::prelude::SliceRandom;
use std::sync::Arc;
use tokio::sync::RwLock;

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

impl Locker {
    pub async fn new(client: reqwest::Client, addr: String) -> Result<Self> {
        let req = format!("https://{}/{}", &addr, "lol-summoner/v1/current-summoner");

        let session: super::core::Session;
        loop {
            let resp = client.get(&req).send().await?.text().await?;
            if let Ok(s) = serde_json::from_str::<super::core::Session>(&resp) {
                session = s;
                break;
            }
            tracing::info!("waiting for the user to log in...");
            let _ = tokio::time::sleep(tokio::time::Duration::from_secs(5));
        }

        println!("we're logged in");

        Ok(Self {
            client,
            session,
            addr,
            ..Default::default()
        })
    }

    fn update_session(&mut self, other: super::core::Session) {
        self.session = other;
    }

    fn update_select(&mut self, other: super::core::Select) {
        self.select = other;
    }

    pub fn summoner_id(&self) -> i64 {
        self.session.summoner_id
    }

    pub fn select(&self) -> super::core::Select {
        let mut select = self.select.clone();

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
    while let Ok(msg) = rx.recv_async().await {
        let summoner_id = state.read().await.summoner_id();

        let selection: super::core::Selection = serde_json::from_value(msg.data)?;
        for player in selection.my_team.iter() {
            if player.summoner_id != summoner_id {
                continue;
            }

            let mut select = super::core::Select::from(player.clone());
            state.write().await.update_select(select);
        }
    }

    Ok(())
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
