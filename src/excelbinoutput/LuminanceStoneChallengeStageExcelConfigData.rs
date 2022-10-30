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

pub type LuminanceStoneChallengeStageExcelConfigData =
    Vec<LuminanceStoneChallengeStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LuminanceStoneChallengeStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "BFNMFEFGECM")]
    pub bfnmfefgecm: i64,

    #[serde(rename = "DAFFNDPLMBM")]
    pub daffndplmbm: i64,

    #[serde(rename = "FPFONIENDMA")]
    pub fpfoniendma: i64,

    #[serde(rename = "MLMMHHCNBKA")]
    pub mlmmhhcnbka: i64,

    #[serde(rename = "CPHIMJDCCCI")]
    pub cphimjdccci: Vec<i64>,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "LFLIICPCIKO")]
    pub lfliicpciko: Vec<i64>,

    #[serde(rename = "IJEEEGCFEHP")]
    pub ijeeegcfehp: i64,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
}
