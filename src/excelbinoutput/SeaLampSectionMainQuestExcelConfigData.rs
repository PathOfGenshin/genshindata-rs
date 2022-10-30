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

pub type SeaLampSectionMainQuestExcelConfigData = Vec<SeaLampSectionMainQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SeaLampSectionMainQuestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "introTextMapHash")]
    pub intro_text_map_hash: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,
}
