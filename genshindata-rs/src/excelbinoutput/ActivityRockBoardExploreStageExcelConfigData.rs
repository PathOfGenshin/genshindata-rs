/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityRockBoardExploreStageExcelConfigData = Vec<ActivityRockBoardExploreStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityRockBoardExploreStageExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "OAPHOJDDJOB")]
    pub oaphojddjob: i64,
    pub open_day: i64,
    #[serde(rename = "CBAMOMDAOFH")]
    pub cbamomdaofh: String,
    pub level_title_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    #[serde(rename = "DPEMPKLKLMN")]
    pub dpempklklmn: i64,
    #[serde(rename = "watcherID")]
    pub watcher_id: i64,
}
