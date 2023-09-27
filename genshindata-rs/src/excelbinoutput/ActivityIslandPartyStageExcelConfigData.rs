/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityIslandPartyStageExcelConfigData = Vec<ActivityIslandPartyStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityIslandPartyStageExcelConfigDatum {
    pub id: i64,
    pub stage_type: String,
    pub gallery_id: i64,
    pub series_id: i64,
    pub match_id: i64,
    pub draft_id: i64,
    pub unlock_day: i64,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub banner_desc_text_map_hash: i64,
    pub fail_hint_text_map_hash: i64,
    pub watcher_list: Vec<i64>,
    #[serde(rename = "scoreIDList")]
    pub score_id_list: Vec<i64>,
    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
}
