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

pub type DungeonRosterConfigData = Vec<DungeonRosterConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DungeonRosterConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openTimeStr")]
    pub open_time_str: String,

    #[serde(rename = "cycleTime")]
    pub cycle_time: i64,

    #[serde(rename = "cycleType")]
    pub cycle_type: String,

    #[serde(rename = "rosterPool")]
    pub roster_pool: Vec<RosterPool>,
}

#[derive(Serialize, Deserialize)]
pub struct RosterPool {
    #[serde(rename = "dungeonList")]
    pub dungeon_list: Vec<i64>,
}
