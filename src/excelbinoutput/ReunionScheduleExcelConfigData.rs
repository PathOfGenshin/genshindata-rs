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

pub type ReunionScheduleExcelConfigData = Vec<ReunionScheduleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReunionScheduleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activateLevel")]
    pub activate_level: i64,

    #[serde(rename = "firstGiftRewardId")]
    pub first_gift_reward_id: i64,

    #[serde(rename = "reunionMissionId")]
    pub reunion_mission_id: i64,

    #[serde(rename = "dailySignInId")]
    pub daily_sign_in_id: i64,

    #[serde(rename = "reunionPrivilegeId")]
    pub reunion_privilege_id: i64,
}
