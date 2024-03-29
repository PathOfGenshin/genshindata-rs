/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExpeditionPathExcelConfigData = Vec<ExpeditionPathExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpeditionPathExcelConfigDatum {
    pub id: i64,
    pub difficulty_id: i64,
    pub super_element: String,
    pub basic_reward_id: i64,
    pub bonus_reward_id: i64,
    pub path_name_text_map_hash: i64,
    pub path_desc_text_map_hash: i64,
    pub level_reward_list: Vec<i64>,
    pub reward_preview_id: i64,
}
