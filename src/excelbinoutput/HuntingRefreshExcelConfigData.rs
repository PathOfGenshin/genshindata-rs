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

pub type HuntingRefreshExcelConfigData = Vec<HuntingRefreshExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HuntingRefreshExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "regionId")]
    pub region_id: i64,

    #[serde(rename = "finishRewardId")]
    pub finish_reward_id: i64,

    #[serde(rename = "difficulty")]
    pub difficulty: Option<Difficulty>,
}

#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "HUNTING_DIFFICULTY_HARD")]
    HuntingDifficultyHard,

    #[serde(rename = "HUNTING_DIFFICULTY_MEDIUM")]
    HuntingDifficultyMedium,
}
