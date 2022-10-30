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

pub type DailyTaskLevelExcelConfigData = Vec<DailyTaskLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DailyTaskLevelExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "minPlayerLevel")]
    pub min_player_level: i64,

    #[serde(rename = "maxPlayerLevel")]
    pub max_player_level: i64,

    #[serde(rename = "scoreDropId")]
    pub score_drop_id: i64,

    #[serde(rename = "scorePreviewRewardId")]
    pub score_preview_reward_id: i64,

    #[serde(rename = "groupReviseLevel")]
    pub group_revise_level: Option<i64>,
}
