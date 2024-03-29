/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityPotionLevelExcelConfigData = Vec<ActivityPotionLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPotionLevelExcelConfigDatum {
    pub level_id: i64,
    pub dungeon_id: i64,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub buff_count_limit: i64,
    pub monster_preview: Vec<i64>,
    pub medal_score_list: Vec<i64>,
    pub medal_time_list_normal: Vec<i64>,
    pub medal_time_list_hard: Vec<i64>,
}
