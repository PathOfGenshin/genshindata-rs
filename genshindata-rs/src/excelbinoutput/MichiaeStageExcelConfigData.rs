/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeStageExcelConfigData = Vec<MichiaeStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MichiaeStageExcelConfigDatum {
    pub stage_id: i64,
    pub open_day: i64,
    pub watcher_list: Vec<i64>,
    pub max_offering_level: i64,
    pub max_crystal_exp: i64,
    pub tab_name_text_map_hash: i64,
}
