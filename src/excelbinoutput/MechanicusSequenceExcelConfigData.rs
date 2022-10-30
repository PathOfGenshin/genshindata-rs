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

pub type MechanicusSequenceExcelConfigData = Vec<MechanicusSequenceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicusSequenceExcelConfigDatum {
    #[serde(rename = "sequenceID")]
    pub sequence_id: i64,

    #[serde(rename = "mechanicusID")]
    pub mechanicus_id: i64,

    #[serde(rename = "openLevel")]
    pub open_level: i64,

    #[serde(rename = "openGearList")]
    pub open_gear_list: Vec<i64>,

    #[serde(rename = "gearLevelLimite")]
    pub gear_level_limite: i64,

    #[serde(rename = "gearMoneyLimite")]
    pub gear_money_limite: i64,

    #[serde(rename = "openMapList")]
    pub open_map_list: Vec<i64>,

    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "rewardPreviewID")]
    pub reward_preview_id: i64,
}
