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

pub type BlossomTalkExcelConfigData = Vec<BlossomTalkExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlossomTalkExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "refreshId")]
    pub refresh_id: i64,

    #[serde(rename = "talkId")]
    pub talk_id: Vec<i64>,

    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,
}
