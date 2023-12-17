/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type NewActivityWatcherConfigData = Vec<NewActivityWatcherConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewActivityWatcherConfigDatum {
    #[serde(rename = "rewardID")]
    pub reward_id: Option<i64>,
    #[serde(rename = "DMOKCGMAHBM")]
    pub dmokcgmahbm: Vec<i64>,
    pub reward_preview: Option<i64>,
    #[serde(rename = "NALHIAEILJK")]
    pub nalhiaeiljk: Vec<i64>,
    pub activitychallengetips_text_map_hash: i64,
    pub extra_activitychallengetips_text_map_hash: i64,
    pub id: i64,
    pub trigger_config: TriggerConfig,
    pub progress: i64,
    pub is_disuse: Option<bool>,
    pub is_auto_grant: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerConfig {
    pub trigger_type: String,
    pub param_list: Vec<String>,
}
