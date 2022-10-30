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

pub type ReunionMissionExcelConfigData = Vec<ReunionMissionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReunionMissionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "watcherGroupId")]
    pub watcher_group_id: i64,

    #[serde(rename = "targetScore")]
    pub target_score: Option<i64>,

    #[serde(rename = "finishRewardId")]
    pub finish_reward_id: Option<i64>,

    #[serde(rename = "PIIKBPMFLON")]
    pub piikbpmflon: Vec<i64>,

    #[serde(rename = "ABMCODFEHKA")]
    pub abmcodfehka: Vec<i64>,

    #[serde(rename = "ECMDGHFCBJN")]
    pub ecmdghfcbjn: Vec<i64>,

    #[serde(rename = "GEGPNJLFNPL")]
    pub gegpnjlfnpl: i64,

    #[serde(rename = "HDLIAKCLGHF")]
    pub hdliakclghf: i64,
}
