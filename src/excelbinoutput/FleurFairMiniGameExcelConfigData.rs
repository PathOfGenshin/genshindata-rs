// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type FleurFairMiniGameExcelConfigData = Vec<FleurFairMiniGameExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FleurFairMiniGameExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "miniGameType")]
    pub mini_game_type: MiniGameType,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "questId")]
    pub quest_id: Option<i64>,

    #[serde(rename = "rewardRankLowTextMapHash")]
    pub reward_rank_low_text_map_hash: i64,

    #[serde(rename = "rewardRankMiddleTextMapHash")]
    pub reward_rank_middle_text_map_hash: i64,

    #[serde(rename = "rewardRankHighTextMapHash")]
    pub reward_rank_high_text_map_hash: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "avatarIdList")]
    pub avatar_id_list: Vec<i64>,

    #[serde(rename = "avatarScore")]
    pub avatar_score: Vec<i64>,

    #[serde(rename = "tipsIdList")]
    pub tips_id_list: Vec<i64>,

    #[serde(rename = "tipsDescTextMapHash")]
    pub tips_desc_text_map_hash: i64,

    #[serde(rename = "pos")]
    pub pos: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum MiniGameType {
    #[serde(rename = "FLEUR_FAIR_MINI_GAME_BALLOON")]
    FleurFairMiniGameBalloon,

    #[serde(rename = "FLEUR_FAIR_MINI_GAME_FALL")]
    FleurFairMiniGameFall,

    #[serde(rename = "FLEUR_FAIR_MINI_GAME_MUSIC")]
    FleurFairMiniGameMusic,
}
