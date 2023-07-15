/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FleurFairDungeonStatExcelConfigData = Vec<FleurFairDungeonStatExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleurFairDungeonStatExcelConfigDatum {
    pub id: i64,
    pub gallery_id: Option<i64>,
    pub stat_type: StatType,
    pub stat_param_list: Vec<String>,
    pub ordering_type: OrderingType,
    pub priority: i64,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub stat_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderingType {
    Equal,
    Greater,
    #[serde(rename = "LessOrEqual")]
    LessOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatType {
    #[serde(rename = "FLEUR_FAIR_DUNGEON_STAT_GROUP_VARIABLE")]
    FleurFairDungeonStatGroupVariable,
    #[serde(rename = "FLEUR_FAIR_DUNGEON_STAT_MONSTER_HURT")]
    FleurFairDungeonStatMonsterHurt,
}
