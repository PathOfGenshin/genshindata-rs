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

pub type LunaRiteQuestExcelConfigData = Vec<LunaRiteQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LunaRiteQuestExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "chapterIcon")]
    pub chapter_icon: String,

    #[serde(rename = "HAEKMEKBCEN")]
    pub haekmekbcen: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Option<i64>,

    #[serde(rename = "LJMLJIODGNJ")]
    pub ljmljiodgnj: Option<i64>,

    #[serde(rename = "KDMDJJNBJLB")]
    pub kdmdjjnbjlb: Option<String>,
}
