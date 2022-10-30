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

pub type DragonSpineStageExcelConfigData = Vec<DragonSpineStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DragonSpineStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "coinIdList")]
    pub coin_id_list: Vec<i64>,

    #[serde(rename = "openday")]
    pub openday: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "missionIdList")]
    pub mission_id_list: Vec<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}
