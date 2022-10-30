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

pub type TowerSkipFloorExcelConfigData = Vec<TowerSkipFloorExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TowerSkipFloorExcelConfigDatum {
    #[serde(rename = "prevLevelIndex")]
    pub prev_level_index: i64,

    #[serde(rename = "curLevelIndex")]
    pub cur_level_index: Option<i64>,
}
