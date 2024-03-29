/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV3TaskExcelConfigData = Vec<LanV3TaskExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanV3TaskExcelConfigDatum {
    pub stage_id: i64,
    pub quest_id: i64,
    pub title_text_map_hash: i64,
    pub open_day: i64,
    pub cond_id: i64,
}
