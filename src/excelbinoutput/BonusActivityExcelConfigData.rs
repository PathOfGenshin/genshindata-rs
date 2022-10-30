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

pub type BonusActivityExcelConfigData = Vec<BonusActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BonusActivityExcelConfigDatum {
    #[serde(rename = "bonusActivityId")]
    pub bonus_activity_id: i64,

    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,

    #[serde(rename = "watcherId")]
    pub watcher_id: Option<i64>,

    #[serde(rename = "rewardItemList")]
    pub reward_item_list: Vec<RewardItemList>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CondList {
    #[serde(rename = "type")]
    pub cond_list_type: Option<String>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct RewardItemList {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}
