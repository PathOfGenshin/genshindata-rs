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

pub type MechanicusExcelConfigData = Vec<MechanicusExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicusExcelConfigDatum {
    #[serde(rename = "mechanicusID")]
    pub mechanicus_id: i64,

    #[serde(rename = "sequenceList")]
    pub sequence_list: Vec<i64>,

    #[serde(rename = "ticketItemID")]
    pub ticket_item_id: i64,

    #[serde(rename = "ticketCostCount")]
    pub ticket_cost_count: i64,

    #[serde(rename = "matchPlayerLimit")]
    pub match_player_limit: i64,

    #[serde(rename = "openGearList")]
    pub open_gear_list: Vec<i64>,

    #[serde(rename = "openMapList")]
    pub open_map_list: Vec<i64>,

    #[serde(rename = "teachDifficultLevel")]
    pub teach_difficult_level: i64,

    #[serde(rename = "punishTime")]
    pub punish_time: i64,
}
