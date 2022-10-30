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

pub type RandTaskExcelConfigData = Vec<RandTaskExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RandTaskExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    #[serde(rename = "reward")]
    pub reward: i64,

    #[serde(rename = "targetTextMapHash")]
    pub target_text_map_hash: i64,

    #[serde(rename = "enterDistance")]
    pub enter_distance: i64,

    #[serde(rename = "exitDistance")]
    pub exit_distance: i64,

    #[serde(rename = "needUI")]
    pub need_ui: Option<bool>,
}
