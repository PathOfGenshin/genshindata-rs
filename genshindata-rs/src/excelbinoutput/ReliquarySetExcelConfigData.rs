/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReliquarySetExcelConfigData = Vec<ReliquarySetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReliquarySetExcelConfigDatum {
    pub set_id: i64,
    pub set_icon: String,
    pub set_need_num: Vec<i64>,
    #[serde(rename = "EquipAffixId")]
    pub equip_affix_id: Option<i64>,
    pub contains_list: Vec<i64>,
    pub bag_sort_value: Option<i64>,
    pub dungeon_group: Vec<i64>,
    pub text_list: Vec<i64>,
    #[serde(rename = "DisableFilter")]
    pub disable_filter: Option<i64>,
}
