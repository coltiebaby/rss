use serde::{Serialize, Deserialize};
use serde_json::value::Value;

// < [8,"OnJsonApiEvent",{"data":[{"championId":63,"childSkins":[],"chromaPreviewPath":null,"disabled":false,"emblems":[],"groupSplash":"","id":63000,"isBase":true,"isChampionUnlocked":true,"name":"Brand","ownership":{"loyaltyReward":false,"owned":true,"rental":{"rented":false},"xboxGPReward":false},"productType":null,"rarityGemPath":"","splashPath":"/lol-game-data/assets/v1/champion-splashes/63/63000.jpg","splashVideoPath":null,"stillObtainable":false,"tilePath":"/lol-game-data/assets/v1/champion-tiles/63/63000.jpg","unlocked":true}],"eventType":"Create","uri":"/lol-champ-select/v1/skin-carousel-skins"}]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselSkin {
    pub champion_id: i64,
    // pub child_skins: Vec<Value>,
    // pub chroma_preview_path: String,
    pub disabled: bool,
    // pub emblems: Vec<Value>,
    // pub group_splash: String,
    pub id: i64,
    // pub is_base: bool,
    // pub is_champion_unlocked: bool,
    // pub name: String,
    // pub ownership: Ownership,
    // pub product_type: Value,
    // pub rarity_gem_path: String,
    // pub splash_path: String,
    // pub splash_video_path: Value,
    // pub still_obtainable: bool,
    // pub tile_path: String,
    pub unlocked: bool,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(default, rename_all = "camelCase")]
// pub struct Ownership {
//     pub loyalty_reward: bool,
//     pub owned: bool,
//     pub rental: Rental,
//     #[serde(rename = "xboxGPReward")]
//     pub xbox_gpreward: bool,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(default, rename_all = "camelCase")]
// pub struct Rental {
//     pub rented: bool,
// }

// [8,"OnJsonApiEvent",{"data":{"championName":"Brand","isSkinGrantedFromBoost":false,"selectedChampionId":63,"selectedSkinId":63004,"showSkinSelector":true,"skinSelectionDisabled":false},"eventType":"Update","uri":"/lol-champ-select/v1/skin-selector-info"}]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selected {
    pub champion_name: String,
    pub is_skin_granted_from_boost: bool,
    pub selected_champion_id: i64,
    pub selected_skin_id: i64,
    pub show_skin_selector: bool,
    pub skin_selection_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    pub selected_skin_id: i64,
    pub spell_1_id: i64,
    pub spell_2_id: i64,
    pub ward_skin_id: i64,
}
