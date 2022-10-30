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

pub type BattlePassMissionExcelConfigData = Vec<BattlePassMissionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BattlePassMissionExcelConfigDatum {
    #[serde(rename = "addPoint")]
    pub add_point: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "guide")]
    pub guide: Guide,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "refreshType")]
    pub refresh_type: Option<RefreshType>,

    #[serde(rename = "scheduleId")]
    pub schedule_id: Option<i64>,

    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,

    #[serde(rename = "isForce")]
    pub is_force: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Guide {
    #[serde(rename = "param")]
    pub param: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum RefreshType {
    #[serde(rename = "BATTLE_PASS_MISSION_REFRESH_CYCLE")]
    BattlePassMissionRefreshCycle,

    #[serde(rename = "BATTLE_PASS_MISSION_REFRESH_CYCLE_CROSS_SCHEDULE")]
    BattlePassMissionRefreshCycleCrossSchedule,

    #[serde(rename = "BATTLE_PASS_MISSION_REFRESH_SCHEDULE")]
    BattlePassMissionRefreshSchedule,
}
