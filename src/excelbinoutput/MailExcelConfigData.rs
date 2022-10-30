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

pub type MailExcelConfigData = Vec<MailExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MailExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "expireDays")]
    pub expire_days: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "senderTextMapHash")]
    pub sender_text_map_hash: i64,

    #[serde(rename = "isStar")]
    pub is_star: Option<bool>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}
