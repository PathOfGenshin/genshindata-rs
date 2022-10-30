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

pub type RandomMainQuestExcelConfigData = Vec<RandomMainQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RandomMainQuestExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_type")]
    pub random_main_quest_excel_config_datum_type: Type,

    #[serde(rename = "_titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "_descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "_luaPath")]
    pub lua_path: String,

    #[serde(rename = "_activeMode")]
    pub active_mode: Option<String>,

    #[serde(rename = "_suggestTrackMainQuestList")]
    pub suggest_track_main_quest_list: Vec<Option<serde_json::Value>>,

    #[serde(rename = "_rewardIdList")]
    pub reward_id_list: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WQ")]
    Wq,
}
