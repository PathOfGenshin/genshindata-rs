/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReunionV2WatcherExcelConfigData = Vec<ReunionV2WatcherExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReunionV2WatcherExcelConfigDatum {
    pub mission_id: i64,
    pub desc_text_map_hash: i64,
    pub reward_id: i64,
    pub id: i64,
    pub trigger_config: TriggerConfig,
    pub progress: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerConfig {
    pub trigger_type: TriggerType,
    pub param_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TriggerType {
    #[serde(rename = "TRIGGER_FINISH_CHAPTER")]
    TriggerFinishChapter,
    #[serde(rename = "TRIGGER_LEVEL1_AREA_EXPLORE_PERCENT")]
    TriggerLevel1AreaExplorePercent,
    #[serde(rename = "TRIGGER_UNLOCK_ANY_SPECIFIC_AREA")]
    TriggerUnlockAnySpecificArea,
}