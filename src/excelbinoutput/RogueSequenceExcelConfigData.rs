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

pub type RogueSequenceExcelConfigData = Vec<RogueSequenceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueSequenceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "JMBECIIBMHM")]
    pub jmbeciibmhm: i64,

    #[serde(rename = "JAEOLDNDANF")]
    pub jaeoldndanf: Vec<i64>,

    #[serde(rename = "BLKENLGBMKB")]
    pub blkenlgbmkb: Blkenlgbmkb,

    #[serde(rename = "HJKECHEKCEN")]
    pub hjkechekcen: Vec<Hjkechekcen>,
}

#[derive(Serialize, Deserialize)]
pub struct Blkenlgbmkb {}

#[derive(Serialize, Deserialize)]
pub struct Hjkechekcen {
    #[serde(rename = "type")]
    pub hjkechekcen_type: Option<Type>,

    #[serde(rename = "BGAEDLFPKHM")]
    pub bgaedlfpkhm: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROGUE_CELL_TYPE_ELITE")]
    RogueCellTypeElite,

    #[serde(rename = "ROGUE_CELL_TYPE_SPRING")]
    RogueCellTypeSpring,

    #[serde(rename = "ROGUE_CELL_TYPE_STORE")]
    RogueCellTypeStore,
}
