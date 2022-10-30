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

pub type EffigyChallengeExcelConfigData = Vec<EffigyChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EffigyChallengeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "prefab")]
    pub prefab: String,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "limitingConditionVec")]
    pub limiting_condition_vec: Vec<i64>,

    #[serde(rename = "upAvatarVec")]
    pub up_avatar_vec: Vec<i64>,

    #[serde(rename = "firstPassRewardId")]
    pub first_pass_reward_id: i64,
}
