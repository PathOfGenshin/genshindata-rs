/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeOfferingDataExcelConfigData = Vec<MichiaeOfferingDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MichiaeOfferingDataExcelConfigDatum {
    pub config_id: i64,
    pub level_new_effect_desc_text_map_hash: i64,
    pub level_new_effect_desc_args: Vec<String>,
    pub level_acc_effect_desc_text_map_hash: i64,
    pub level_acc_effect_desc_args: Vec<String>,
    pub level: Option<i64>,
}
