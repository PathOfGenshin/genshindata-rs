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

pub type ChatExcelConfigData = Vec<ChatExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChatExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "priority")]
    pub priority: f64,

    #[serde(rename = "tabShownNameTextMapHash")]
    pub tab_shown_name_text_map_hash: i64,

    #[serde(rename = "chatChannelIcon")]
    pub chat_channel_icon: String,

    #[serde(rename = "TagOtherTextMapHash")]
    pub tag_other_text_map_hash: i64,

    #[serde(rename = "TagSelfTextMapHash")]
    pub tag_self_text_map_hash: i64,

    #[serde(rename = "EnterTextMapHash")]
    pub enter_text_map_hash: i64,

    #[serde(rename = "LeaveTextMapHash")]
    pub leave_text_map_hash: i64,

    #[serde(rename = "channel")]
    pub channel: Option<i64>,
}
