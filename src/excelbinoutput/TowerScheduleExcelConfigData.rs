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

pub type TowerScheduleExcelConfigData = Vec<TowerScheduleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TowerScheduleExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "entranceFloorId")]
    pub entrance_floor_id: Vec<i64>,

    #[serde(rename = "schedules")]
    pub schedules: Vec<Schedule>,

    #[serde(rename = "closeTime")]
    pub close_time: String,

    #[serde(rename = "IEPBHNILIDB")]
    pub iepbhnilidb: i64,

    #[serde(rename = "scheduleRewards")]
    pub schedule_rewards: Vec<ScheduleReward>,

    #[serde(rename = "monthlyLevelConfigId")]
    pub monthly_level_config_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "buffnameTextMapHash")]
    pub buffname_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScheduleReward {
    #[serde(rename = "minStarCount")]
    pub min_star_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "floorList")]
    pub floor_list: Vec<i64>,

    #[serde(rename = "openTime")]
    pub open_time: String,
}
