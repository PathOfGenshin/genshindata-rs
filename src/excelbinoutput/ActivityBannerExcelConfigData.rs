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

pub type ActivityBannerExcelConfigData = Vec<ActivityBannerExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityBannerExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "INPFCDCCLON")]
    pub inpfcdcclon: Option<String>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,

    #[serde(rename = "CKPJFAFKDFK")]
    pub ckpjfafkdfk: String,

    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "HHOJIAAHDMK")]
    pub hhojiaahdmk: Option<bool>,
}
