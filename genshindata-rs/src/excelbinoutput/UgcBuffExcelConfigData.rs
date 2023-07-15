/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcBuffExcelConfigData = Vec<UgcBuffExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UgcBuffExcelConfigDatum {
    pub buff_id: i64,
    pub buff_name_text_map_hash: i64,
    #[serde(rename = "BHPHILEGLNA")]
    pub bhphileglna: i64,
    pub desc_param: Vec<String>,
    pub ability_name: String,
}