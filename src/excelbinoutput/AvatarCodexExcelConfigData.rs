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

pub type AvatarCodexExcelConfigData = Vec<AvatarCodexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarCodexExcelConfigDatum {
    #[serde(rename = "sortId")]
    pub sort_id: i64,

    #[serde(rename = "sortFactor")]
    pub sort_factor: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "beginTime")]
    pub begin_time: String,

    #[serde(rename = "IMPOGOCCALB")]
    pub impogoccalb: Option<bool>,
}
