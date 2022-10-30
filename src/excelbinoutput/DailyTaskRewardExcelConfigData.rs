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

pub type DailyTaskRewardExcelConfigData = Vec<DailyTaskRewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DailyTaskRewardExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "dropVec")]
    pub drop_vec: Vec<DropVec>,
}

#[derive(Serialize, Deserialize)]
pub struct DropVec {
    #[serde(rename = "dropId")]
    pub drop_id: i64,

    #[serde(rename = "previewRewardId")]
    pub preview_reward_id: i64,
}
