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

pub type ReputationCityExcelConfigData = Vec<ReputationCityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReputationCityExcelConfigDatum {
    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "exploreAreaVec")]
    pub explore_area_vec: Vec<i64>,

    #[serde(rename = "virtualItemId")]
    pub virtual_item_id: i64,

    #[serde(rename = "openState")]
    pub open_state: String,

    #[serde(rename = "bgIconPath")]
    pub bg_icon_path: String,

    #[serde(rename = "bgEffectPath")]
    pub bg_effect_path: String,

    #[serde(rename = "explainTitleTextMapHash")]
    pub explain_title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "rewardItemId")]
    pub reward_item_id: i64,

    #[serde(rename = "rewardItemDescTextMapHash")]
    pub reward_item_desc_text_map_hash: i64,

    #[serde(rename = "rewardItemIcon")]
    pub reward_item_icon: String,

    #[serde(rename = "rewardBgIcon")]
    pub reward_bg_icon: String,

    #[serde(rename = "cityIcon")]
    pub city_icon: String,
}
