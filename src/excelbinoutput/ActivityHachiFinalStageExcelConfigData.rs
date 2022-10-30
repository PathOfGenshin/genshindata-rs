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

pub type ActivityHachiFinalStageExcelConfigData = Vec<ActivityHachiFinalStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityHachiFinalStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "DGGMIFJMJBP")]
    pub dggmifjmjbp: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "GBJADHAELIL")]
    pub gbjadhaelil: i64,

    #[serde(rename = "NPPHECBAGMH")]
    pub npphecbagmh: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "PLDINHEEGEG")]
    pub pldinheegeg: Vec<i64>,

    #[serde(rename = "LBOKGFIMAMN")]
    pub lbokgfimamn: i64,
}
