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

pub type FlightActivityExcelConfigData = Vec<FlightActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FlightActivityExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "previewRewardId")]
    pub preview_reward_id: i64,

    #[serde(rename = "startQuestId")]
    pub start_quest_id: i64,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "medalId")]
    pub medal_id: Vec<i64>,

    #[serde(rename = "dailyFactorVec")]
    pub daily_factor_vec: Vec<DailyFactorVec>,
}

#[derive(Serialize, Deserialize)]
pub struct DailyFactorVec {
    #[serde(rename = "timeFactor")]
    pub time_factor: i64,

    #[serde(rename = "goldFactor")]
    pub gold_factor: i64,
}
