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

pub type BargainExcelConfigData = Vec<BargainExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BargainExcelConfigDatum {
    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dialogId")]
    pub dialog_id: Vec<i64>,

    #[serde(rename = "expectedValue")]
    pub expected_value: Vec<i64>,

    #[serde(rename = "space")]
    pub space: i64,

    #[serde(rename = "successTalkId")]
    pub success_talk_id: Vec<i64>,

    #[serde(rename = "failTalkId")]
    pub fail_talk_id: i64,

    #[serde(rename = "moodNpcId")]
    pub mood_npc_id: i64,

    #[serde(rename = "moodUpperLimit")]
    pub mood_upper_limit: i64,

    #[serde(rename = "randomMood")]
    pub random_mood: Vec<i64>,

    #[serde(rename = "moodAlertLimit")]
    pub mood_alert_limit: i64,

    #[serde(rename = "moodLowLimit")]
    pub mood_low_limit: i64,

    #[serde(rename = "moodLowLimitTextTextMapHash")]
    pub mood_low_limit_text_text_map_hash: i64,

    #[serde(rename = "singleFailMoodDeduction")]
    pub single_fail_mood_deduction: i64,

    #[serde(rename = "singleFailTalkId")]
    pub single_fail_talk_id: Vec<i64>,

    #[serde(rename = "deleteItem")]
    pub delete_item: Option<bool>,

    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "titleTextTextMapHash")]
    pub title_text_text_map_hash: i64,

    #[serde(rename = "affordTextTextMapHash")]
    pub afford_text_text_map_hash: i64,

    #[serde(rename = "storageTextTextMapHash")]
    pub storage_text_text_map_hash: i64,

    #[serde(rename = "moodHintTextTextMapHash")]
    pub mood_hint_text_text_map_hash: i64,

    #[serde(rename = "moodDescTextTextMapHash")]
    pub mood_desc_text_text_map_hash: i64,
}
