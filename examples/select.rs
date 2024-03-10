#[tokio::main]
async fn main() {
    let lc = lcu::client::Client::new().unwrap();
    let selected = lcu::core::champion_select::skins::Select {
        spell_1_id: 14,
        spell_2_id: 4,
        selected_skin_id: 63000,
        ward_skin_id: 1,
    };

    let data = serde_json::to_string(&selected).unwrap();

    lc.patch("lol-champ-select/v1/session/my-selection", data).await;
}
