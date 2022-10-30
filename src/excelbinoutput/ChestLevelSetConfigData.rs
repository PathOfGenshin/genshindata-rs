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

pub type ChestLevelSetConfigData = Vec<ChestLevelSetConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChestLevelSetConfigDatum {
    #[serde(rename = "chestLevel")]
    pub chest_level: i64,

    #[serde(rename = "zoneMinLevel")]
    pub zone_min_level: i64,
}
