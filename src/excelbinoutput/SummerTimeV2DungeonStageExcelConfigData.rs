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

pub type SummerTimeV2DungeonStageExcelConfigData = Vec<SummerTimeV2DungeonStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SummerTimeV2DungeonStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "prevDungeonId")]
    pub prev_dungeon_id: i64,

    #[serde(rename = "DPPCHIBBEPN")]
    pub dppchibbepn: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "LKDGHJOBNAD")]
    pub lkdghjobnad: i64,

    #[serde(rename = "CACJEKMINCM")]
    pub cacjekmincm: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "LBOKGFIMAMN")]
    pub lbokgfimamn: i64,

    #[serde(rename = "HNNKICHABNI")]
    pub hnnkichabni: i64,

    #[serde(rename = "PLKMNLIPABK")]
    pub plkmnlipabk: Option<i64>,

    #[serde(rename = "PLKDJJKAHPO")]
    pub plkdjjkahpo: Option<i64>,

    #[serde(rename = "OMNOPJGNMJE")]
    pub omnopjgnmje: Vec<i64>,

    #[serde(rename = "POGFJFMMOMN")]
    pub pogfjfmmomn: Vec<i64>,

    #[serde(rename = "COCJPDDBKCN")]
    pub cocjpddbkcn: i64,

    #[serde(rename = "AJGMMBKAKHA")]
    pub ajgmmbkakha: i64,

    #[serde(rename = "GNNJLIEOHJB")]
    pub gnnjlieohjb: i64,

    #[serde(rename = "HNLHIHHNFDG")]
    pub hnlhihhnfdg: i64,
}
