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

pub type CoopInteractionExcelConfigData = Vec<CoopInteractionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CoopInteractionExcelConfigDatum {
    #[serde(rename = "idRawNum")]
    pub id_raw_num: i64,

    #[serde(rename = "npcIdRawNum")]
    pub npc_id_raw_num: i64,

    #[serde(rename = "mainQuestIdRawNum")]
    pub main_quest_id_raw_num: i64,

    #[serde(rename = "priorityRawNum")]
    pub priority_raw_num: i64,

    #[serde(rename = "_isAuto")]
    pub is_auto: Option<bool>,
}
