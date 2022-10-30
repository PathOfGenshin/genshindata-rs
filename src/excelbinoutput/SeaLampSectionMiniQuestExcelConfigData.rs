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

pub type SeaLampSectionMiniQuestExcelConfigData = Vec<SeaLampSectionMiniQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SeaLampSectionMiniQuestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sectionId")]
    pub section_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "miniQuestId")]
    pub mini_quest_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "isSpecial")]
    pub is_special: Option<bool>,
}
