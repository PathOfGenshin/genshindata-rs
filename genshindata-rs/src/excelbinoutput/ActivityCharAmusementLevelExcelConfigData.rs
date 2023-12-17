/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityCharAmusementLevelExcelConfigData = Vec<ActivityCharAmusementLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityCharAmusementLevelExcelConfigDatum {
    pub level_id: i64,
    pub weight: i64,
    pub level_type: LevelType,
    pub gallery_id: i64,
    #[serde(rename = "MNGNJBMAGFG")]
    pub mngnjbmagfg: i64,
    #[serde(rename = "PMKLKNBLEBG")]
    pub pmklknblebg: i64,
    pub trial_avatar_list: Vec<i64>,
    #[serde(rename = "DCMGMLGNIAP")]
    pub dcmgmlgniap: String,
    pub level_name_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    pub level_rule_text_map_hash: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LevelType {
    #[serde(rename = "LEVEL_TYPE_AMUSEMENT")]
    LevelTypeAmusement,
    #[serde(rename = "LEVEL_TYPE_FIGHT")]
    LevelTypeFight,
}
