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

pub type DungeonSerialConfigData = Vec<DungeonSerialConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DungeonSerialConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MaxTakeNum")]
    pub max_take_num: i64,

    #[serde(rename = "takeCost")]
    pub take_cost: Option<i64>,
}
