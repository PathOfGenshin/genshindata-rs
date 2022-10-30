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

pub type WinterCampExploreExcelConfigData = Vec<WinterCampExploreExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WinterCampExploreExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "groupID")]
    pub group_id: i64,

    #[serde(rename = "PGGOKANNJLJ")]
    pub pggokannjlj: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "FLGLMAIBLNH")]
    pub flglmaiblnh: Vec<Flglmaiblnh>,
}

#[derive(Serialize, Deserialize)]
pub struct Flglmaiblnh {
    #[serde(rename = "dropId")]
    pub drop_id: Vec<i64>,
}
