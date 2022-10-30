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

pub type SummerTimeV2BoatStageExcelConfigData = Vec<SummerTimeV2BoatStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SummerTimeV2BoatStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "PGGOKANNJLJ")]
    pub pggokannjlj: i64,

    #[serde(rename = "conditionType")]
    pub condition_type: Vec<ConditionType>,

    #[serde(rename = "FKKPOIAEOBJ")]
    pub fkkpoiaeobj: Vec<i64>,

    #[serde(rename = "KDAJANAFILK")]
    pub kdajanafilk: Vec<i64>,

    #[serde(rename = "ABMOLAJNEHE")]
    pub abmolajnehe: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionType {
    #[serde(rename = "PARAM")]
    Param,

    #[serde(rename = "TIME")]
    Time,
}
