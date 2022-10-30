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

pub type ActivitySummerTimeStageExcelConfigData = Vec<ActivitySummerTimeStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySummerTimeStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "mainQuest")]
    pub main_quest: i64,

    #[serde(rename = "HJHGKGJGBIG")]
    pub hjhgkgjgbig: Option<i64>,

    #[serde(rename = "OHALKBDDELB")]
    pub ohalkbddelb: Option<i64>,

    #[serde(rename = "DGGMIFJMJBP")]
    pub dggmifjmjbp: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "LKLOOPJJNNI")]
    pub lkloopjjnni: i64,

    #[serde(rename = "DKGNJHDIODG")]
    pub dkgnjhdiodg: i64,

    #[serde(rename = "preQuest")]
    pub pre_quest: Option<i64>,
}
