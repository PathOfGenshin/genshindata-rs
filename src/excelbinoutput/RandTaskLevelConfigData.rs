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

pub type RandTaskLevelConfigData = Vec<RandTaskLevelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RandTaskLevelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "minZoneLevel")]
    pub min_zone_level: i64,

    #[serde(rename = "maxZoneLevel")]
    pub max_zone_level: i64,

    #[serde(rename = "reviseLevel")]
    pub revise_level: Option<i64>,
}
