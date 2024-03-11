use serde::{Serialize, Deserialize};
use serde_json::value::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub summoner_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselSkin {
    pub champion_id: i64,
    pub disabled: bool,
    pub id: i64,
    pub unlocked: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selection {
    pub my_team: Vec<MyTeam>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTeam {
    #[serde(rename = "spell1Id")]
    pub spell1id: i64,
    #[serde(rename = "spell2Id")]
    pub spell2id: i64,
    pub summoner_id: i64,
    pub ward_skin_id: i64,
}

impl From<MyTeam> for Select {
    fn from(value: MyTeam) -> Self {
        Self {
            spell1id: value.spell1id,
            spell2id: value.spell2id,
            ward_skin_id: value.ward_skin_id,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    pub selected_skin_id: i64,
    #[serde(rename = "spel11Id")]
    pub spell1id: i64,
    #[serde(rename = "spell2Id")]
    pub spell2id: i64,
    pub ward_skin_id: i64,
}
