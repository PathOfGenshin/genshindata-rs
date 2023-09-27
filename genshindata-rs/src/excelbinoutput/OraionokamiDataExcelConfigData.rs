/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type OraionokamiDataExcelConfigData = Vec<OraionokamiDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OraionokamiDataExcelConfigDatum {
    pub config_id: i64,
    pub gadget_chain_id: i64,
    pub gadget_chain_level: i64,
    pub server_buff_id: i64,
    pub desc_id_list: Vec<i64>,
    pub desc_title_text_map_hash: i64,
    pub desc_content_text_map_hash: i64,
    pub desc_skill_text_map_hash: i64,
    pub group_id: i64,
    pub type_level: i64,
    pub icon_path: String,
}
