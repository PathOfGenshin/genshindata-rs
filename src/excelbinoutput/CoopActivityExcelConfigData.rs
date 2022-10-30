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

pub type CoopActivityExcelConfigData = Vec<CoopActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CoopActivityExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "activityNameTextMapHash")]
    pub activity_name_text_map_hash: i64,

    #[serde(rename = "activityDesTextMapHash")]
    pub activity_des_text_map_hash: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: Option<i64>,
}
