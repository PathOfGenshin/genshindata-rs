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

pub type BattlePassRewardExcelConfigData = Vec<BattlePassRewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BattlePassRewardExcelConfigDatum {
    #[serde(rename = "indexId")]
    pub index_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "freeRewardIdList")]
    pub free_reward_id_list: Vec<i64>,

    #[serde(rename = "paidRewardIdList")]
    pub paid_reward_id_list: Vec<i64>,

    #[serde(rename = "showUp")]
    pub show_up: Option<bool>,
}
