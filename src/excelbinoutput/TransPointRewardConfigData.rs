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

pub type TransPointRewardConfigData = Vec<TransPointRewardConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TransPointRewardConfigDatum {
    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "pointId")]
    pub point_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "PGLALHMBDOH")]
    pub pglalhmbdoh: Vec<Option<serde_json::Value>>,

    #[serde(rename = "groupIdVec")]
    pub group_id_vec: Vec<i64>,
}
