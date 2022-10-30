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

pub type GivingGroupExcelConfigData = Vec<GivingGroupExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GivingGroupExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "ItemIds")]
    pub item_ids: Vec<i64>,

    #[serde(rename = "finishTalkId")]
    pub finish_talk_id: Option<i64>,

    #[serde(rename = "mistakeTalkId")]
    pub mistake_talk_id: Option<i64>,

    #[serde(rename = "HIDNEKHDODL")]
    pub hidnekhdodl: Option<i64>,
}
