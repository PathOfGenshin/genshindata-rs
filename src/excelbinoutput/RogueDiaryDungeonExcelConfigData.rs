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

pub type RogueDiaryDungeonExcelConfigData = Vec<RogueDiaryDungeonExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryDungeonExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "groupList")]
    pub group_list: Vec<i64>,

    #[serde(rename = "ADEOKCKMKEE")]
    pub adeokckmkee: Vec<Adeokckmkee>,
}

#[derive(Serialize, Deserialize)]
pub struct Adeokckmkee {
    #[serde(rename = "FFNMDMHMFNP")]
    pub ffnmdmhmfnp: Option<i64>,

    #[serde(rename = "FBPLKKGEPAC")]
    pub fbplkkgepac: Vec<i64>,

    #[serde(rename = "HDHNOHIFPDE")]
    pub hdhnohifpde: Option<i64>,
}
