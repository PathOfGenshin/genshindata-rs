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

pub type ReputationExploreExcelConfigData = Vec<ReputationExploreExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReputationExploreExcelConfigDatum {
    #[serde(rename = "exploreId")]
    pub explore_id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "exploreProgress")]
    pub explore_progress: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "conditionTextTextMapHash")]
    pub condition_text_text_map_hash: i64,
}
