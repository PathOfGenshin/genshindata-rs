/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySpiceStageDataExcelConfigData = Vec<ActivitySpiceStageDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySpiceStageDataExcelConfigDatum {
    pub id: i64,
    pub open_day_index: i64,
    pub fetter_inc_cnt_limit: i64,
    pub material_id_list: Vec<i64>,
    pub material_num_list: Vec<i64>,
    pub material_order_list: Vec<i64>,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub speed: i64,
    pub times: i64,
    pub click_num: i64,
    pub click_zone: Vec<i64>,
    pub watcher_id: i64,
}
