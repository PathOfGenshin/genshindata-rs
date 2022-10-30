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

pub type LevelTagResetExcelConfigData = Vec<LevelTagResetExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LevelTagResetExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "LBLOAJNHCFB")]
    pub lbloajnhcfb: Vec<i64>,
}
