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

pub type ActivitySalesmanExcelConfigData = Vec<ActivitySalesmanExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySalesmanExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "dailyConfigIdList")]
    pub daily_config_id_list: Vec<i64>,

    #[serde(rename = "normalRewardIdList")]
    pub normal_reward_id_list: Vec<i64>,

    #[serde(rename = "specialRewardIdList")]
    pub special_reward_id_list: Vec<i64>,

    #[serde(rename = "specialProbList")]
    pub special_prob_list: Vec<f64>,

    #[serde(rename = "specialReward")]
    pub special_reward: SpecialReward,
}

#[derive(Serialize, Deserialize)]
pub struct SpecialReward {
    #[serde(rename = "obtainParam")]
    pub obtain_param: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "rewardType")]
    pub reward_type: Option<String>,

    #[serde(rename = "obtainMethod")]
    pub obtain_method: Option<String>,

    #[serde(rename = "previewId")]
    pub preview_id: Option<i64>,
}
