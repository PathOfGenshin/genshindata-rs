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

pub type ActivitySummerTimeExcelConfigData = Vec<ActivitySummerTimeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySummerTimeExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "unlockQuestID")]
    pub unlock_quest_id: i64,

    #[serde(rename = "contentDuration")]
    pub content_duration: i64,

    #[serde(rename = "unlockPlayerLevel")]
    pub unlock_player_level: i64,

    #[serde(rename = "FIPDBBFPGMD")]
    pub fipdbbfpgmd: i64,

    #[serde(rename = "CHLAMMFJKAP")]
    pub chlammfjkap: Vec<i64>,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,
}
