/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EquipAffixExcelConfigData = Vec<EquipAffixExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipAffixExcelConfigDatum {
    pub affix_id: i64,
    pub id: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub open_config: String,
    pub add_props: Vec<AddProp>,
    pub param_list: Vec<f64>,
    pub level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddProp {
    pub prop_type: Option<String>,
    pub value: Option<f64>,
}
