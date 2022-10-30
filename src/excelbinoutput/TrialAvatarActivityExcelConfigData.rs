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

pub type TrialAvatarActivityExcelConfigData = Vec<TrialAvatarActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TrialAvatarActivityExcelConfigDatum {
    #[serde(rename = "ScheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "AvatarIndexIdList")]
    pub avatar_index_id_list: Vec<i64>,

    #[serde(rename = "RewardIdList")]
    pub reward_id_list: Vec<i64>,
}
