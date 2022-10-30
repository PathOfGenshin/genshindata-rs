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

pub type HuntingClueGatherExcelConfigData = Vec<HuntingClueGatherExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HuntingClueGatherExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "gatherId")]
    pub gather_id: i64,

    #[serde(rename = "gatherGroupId")]
    pub gather_group_id: i64,

    #[serde(rename = "isClueGather")]
    pub is_clue_gather: bool,
}
