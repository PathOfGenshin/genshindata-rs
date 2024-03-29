/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityDeliveryWatcherDataConfigData = Vec<ActivityDeliveryWatcherDataConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDeliveryWatcherDataConfigDatum {
    pub id: i64,
    pub trigger_config: TriggerConfig,
    pub progress: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerConfig {
    pub trigger_type: String,
    pub param_list: Vec<String>,
}
