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

pub type ReputationLevelExcelConfigData = Vec<ReputationLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReputationLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "nextLevelExp")]
    pub next_level_exp: Option<i64>,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "requestGroupId")]
    pub request_group_id: i64,

    #[serde(rename = "requestNum")]
    pub request_num: i64,

    #[serde(rename = "requestAcceptNum")]
    pub request_accept_num: i64,

    #[serde(rename = "functionId")]
    pub function_id: Option<i64>,
}
